use {
    anchor_lang::{
        prelude::Pubkey, solana_program::instruction::Instruction, AccountDeserialize,
        InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

fn program_bytes() -> Vec<u8> {
    let so_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy/scholarfi_lite_contracts.so");
    std::fs::read(&so_path).unwrap_or_else(|e| {
        panic!(
            "failed to read program binary at {}: {e}. Run `anchor build` first.",
            so_path.display()
        )
    })
}

fn send_ix(
    svm: &mut LiteSVM,
    payer: &Keypair,
    instruction: Instruction,
) -> Result<litesvm::types::TransactionMetadata, litesvm::types::FailedTransactionMetadata> {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
    svm.send_transaction(tx)
}

fn student_list_pda(program_id: &Pubkey, teacher: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            scholarfi_lite_contracts::constants::STUDENT_LIST_SEED,
            teacher.as_ref(),
        ],
        program_id,
    )
}

fn course_pda(program_id: &Pubkey, teacher: &Pubkey, course_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            scholarfi_lite_contracts::constants::COURSE_SEED,
            teacher.as_ref(),
            &course_id.to_le_bytes(),
        ],
        program_id,
    )
}

fn achievement_pda(
    program_id: &Pubkey,
    teacher: &Pubkey,
    student: &Pubkey,
    achievement_id: u64,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            scholarfi_lite_contracts::constants::ACHIEVEMENT_SEED,
            teacher.as_ref(),
            student.as_ref(),
            &achievement_id.to_le_bytes(),
        ],
        program_id,
    )
}

#[test]
fn test_student_list_crud() {
    let program_id = scholarfi_lite_contracts::id();
    let teacher = Keypair::new();
    let student_a = Keypair::new();
    let student_b = Keypair::new();
    let mut svm = LiteSVM::new();
    svm.add_program(program_id, &program_bytes()).unwrap();
    svm.airdrop(&teacher.pubkey(), 2_000_000_000).unwrap();

    let (list_pda, _) = student_list_pda(&program_id, &teacher.pubkey());

    // Create
    let create_ix = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::CreateStudentList {
            students: vec![student_a.pubkey()],
        }
        .data(),
        scholarfi_lite_contracts::accounts::CreateStudentList {
            teacher: teacher.pubkey(),
            student_list: list_pda,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, create_ix).is_ok());

    let list = scholarfi_lite_contracts::state::StudentList::try_deserialize(
        &mut svm.get_account(&list_pda).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(list.students.len(), 1);
    assert_eq!(list.students[0], student_a.pubkey());

    // Add
    let add_ix = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::AddStudent {
            student: student_b.pubkey(),
        }
        .data(),
        scholarfi_lite_contracts::accounts::AddStudent {
            teacher: teacher.pubkey(),
            student_list: list_pda,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, add_ix).is_ok());

    let list = scholarfi_lite_contracts::state::StudentList::try_deserialize(
        &mut svm.get_account(&list_pda).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(list.students.len(), 2);
    assert!(list.students.contains(&student_b.pubkey()));

    // Remove
    let remove_ix = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::RemoveStudent {
            student: student_a.pubkey(),
        }
        .data(),
        scholarfi_lite_contracts::accounts::RemoveStudent {
            teacher: teacher.pubkey(),
            student_list: list_pda,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, remove_ix).is_ok());

    let list = scholarfi_lite_contracts::state::StudentList::try_deserialize(
        &mut svm.get_account(&list_pda).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(list.students, vec![student_b.pubkey()]);

    // Close
    let close_ix = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::CloseStudentList {}.data(),
        scholarfi_lite_contracts::accounts::CloseStudentList {
            teacher: teacher.pubkey(),
            student_list: list_pda,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, close_ix).is_ok());
    assert!(svm.get_account(&list_pda).is_none());
}

#[test]
fn test_course_crud() {
    let program_id = scholarfi_lite_contracts::id();
    let teacher = Keypair::new();
    let mut svm = LiteSVM::new();
    svm.add_program(program_id, &program_bytes()).unwrap();
    svm.airdrop(&teacher.pubkey(), 2_000_000_000).unwrap();

    let course_id = 42u64;
    let (course_addr, _) = course_pda(&program_id, &teacher.pubkey(), course_id);

    let create_ix = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::CreateCourse {
            course_id,
            title: "Intro to Solana".to_string(),
        }
        .data(),
        scholarfi_lite_contracts::accounts::CreateCourse {
            teacher: teacher.pubkey(),
            course: course_addr,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, create_ix).is_ok());

    let course = scholarfi_lite_contracts::state::Course::try_deserialize(
        &mut svm.get_account(&course_addr).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(course.title, "Intro to Solana");
    assert!(course.active);

    let update_ix = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::UpdateCourse {
            course_id,
            title: "Solana Advanced".to_string(),
            active: false,
        }
        .data(),
        scholarfi_lite_contracts::accounts::UpdateCourse {
            teacher: teacher.pubkey(),
            course: course_addr,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, update_ix).is_ok());

    let course = scholarfi_lite_contracts::state::Course::try_deserialize(
        &mut svm.get_account(&course_addr).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(course.title, "Solana Advanced");
    assert!(!course.active);

    let close_ix = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::CloseCourse { course_id }.data(),
        scholarfi_lite_contracts::accounts::CloseCourse {
            teacher: teacher.pubkey(),
            course: course_addr,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, close_ix).is_ok());
    assert!(svm.get_account(&course_addr).is_none());
}

#[test]
fn test_achievement_crud_and_not_on_list() {
    let program_id = scholarfi_lite_contracts::id();
    let teacher = Keypair::new();
    let student = Keypair::new();
    let outsider = Keypair::new();
    let mut svm = LiteSVM::new();
    svm.add_program(program_id, &program_bytes()).unwrap();
    svm.airdrop(&teacher.pubkey(), 5_000_000_000).unwrap();

    let (list_pda, _) = student_list_pda(&program_id, &teacher.pubkey());
    let course_id = 7u64;
    let (course_addr, _) = course_pda(&program_id, &teacher.pubkey(), course_id);
    let achievement_id = 1u64;
    let (achievement_addr, _) =
        achievement_pda(&program_id, &teacher.pubkey(), &student.pubkey(), achievement_id);

    // Setup list + course
    let create_list = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::CreateStudentList {
            students: vec![student.pubkey()],
        }
        .data(),
        scholarfi_lite_contracts::accounts::CreateStudentList {
            teacher: teacher.pubkey(),
            student_list: list_pda,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, create_list).is_ok());

    let create_course = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::CreateCourse {
            course_id,
            title: "Credentials 101".to_string(),
        }
        .data(),
        scholarfi_lite_contracts::accounts::CreateCourse {
            teacher: teacher.pubkey(),
            course: course_addr,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, create_course).is_ok());

    // Fail: student not on list
    let (outsider_ach, _) =
        achievement_pda(&program_id, &teacher.pubkey(), &outsider.pubkey(), achievement_id);
    let bad_register = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::RegisterAchievement {
            achievement_id,
            title: "Hackathon".to_string(),
            achievement_type: scholarfi_lite_contracts::state::AchievementType::Nft,
            course_id: Some(course_id),
        }
        .data(),
        scholarfi_lite_contracts::accounts::RegisterAchievement {
            teacher: teacher.pubkey(),
            student: outsider.pubkey(),
            student_list: list_pda,
            course: Some(course_addr),
            achievement: outsider_ach,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, bad_register).is_err());

    // Register linked to course
    let register = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::RegisterAchievement {
            achievement_id,
            title: "Completed module".to_string(),
            achievement_type: scholarfi_lite_contracts::state::AchievementType::Token,
            course_id: Some(course_id),
        }
        .data(),
        scholarfi_lite_contracts::accounts::RegisterAchievement {
            teacher: teacher.pubkey(),
            student: student.pubkey(),
            student_list: list_pda,
            course: Some(course_addr),
            achievement: achievement_addr,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, register).is_ok());

    let achievement = scholarfi_lite_contracts::state::Achievement::try_deserialize(
        &mut svm.get_account(&achievement_addr).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(achievement.title, "Completed module");
    assert_eq!(achievement.course_id, Some(course_id));

    // Update
    let update = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::UpdateAchievement {
            achievement_id,
            title: "Completed module with honors".to_string(),
            achievement_type: scholarfi_lite_contracts::state::AchievementType::Nft,
            course_id: Some(course_id),
        }
        .data(),
        scholarfi_lite_contracts::accounts::UpdateAchievement {
            teacher: teacher.pubkey(),
            student: student.pubkey(),
            student_list: list_pda,
            course: Some(course_addr),
            achievement: achievement_addr,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, update).is_ok());

    let achievement = scholarfi_lite_contracts::state::Achievement::try_deserialize(
        &mut svm.get_account(&achievement_addr).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(achievement.title, "Completed module with honors");
    assert_eq!(
        achievement.achievement_type,
        scholarfi_lite_contracts::state::AchievementType::Nft
    );

    // Delete
    let delete = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::DeleteAchievement { achievement_id }.data(),
        scholarfi_lite_contracts::accounts::DeleteAchievement {
            teacher: teacher.pubkey(),
            student: student.pubkey(),
            student_list: list_pda,
            achievement: achievement_addr,
        }
        .to_account_metas(None),
    );
    assert!(send_ix(&mut svm, &teacher, delete).is_ok());
    assert!(svm.get_account(&achievement_addr).is_none());
}
