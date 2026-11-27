use {
    anchor_lang::{
        prelude::Pubkey, solana_program::instruction::Instruction, AccountDeserialize,
        InstructionData, ToAccountMetas,
    },
    anchor_spl::associated_token::get_associated_token_address_with_program_id,
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

use scholarfi_lite_contracts::state::{AchievementType, RewardConfig, RewardReceipt};

fn program_bytes() -> Vec<u8> {
    let so_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/deploy/scholarfi_lite_contracts.so");
    std::fs::read(&so_path).unwrap_or_else(|e| {
        panic!(
            "failed to read program binary at {}: {e}. Run `anchor build` / cargo-build-sbf --arch v1 first.",
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

fn pda(program_id: &Pubkey, seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, program_id).0
}

fn student_list_pda(program_id: &Pubkey, teacher: &Pubkey) -> Pubkey {
    pda(
        program_id,
        &[
            scholarfi_lite_contracts::constants::STUDENT_LIST_SEED,
            teacher.as_ref(),
        ],
    )
}

fn reward_mint_pda(program_id: &Pubkey, teacher: &Pubkey, mint_id: u64) -> Pubkey {
    pda(
        program_id,
        &[
            scholarfi_lite_contracts::constants::REWARD_MINT_SEED,
            teacher.as_ref(),
            &mint_id.to_le_bytes(),
        ],
    )
}

fn reward_config_pda(program_id: &Pubkey, teacher: &Pubkey) -> Pubkey {
    pda(
        program_id,
        &[
            scholarfi_lite_contracts::constants::REWARD_CONFIG_SEED,
            teacher.as_ref(),
        ],
    )
}

fn setup() -> (LiteSVM, Keypair, Pubkey) {
    let program_id = scholarfi_lite_contracts::id();
    let teacher = Keypair::new();
    let mut svm = LiteSVM::new();
    svm.add_program(program_id, &program_bytes()).unwrap();
    svm.airdrop(&teacher.pubkey(), 5_000_000_000).unwrap();
    (svm, teacher, program_id)
}

fn create_list(
    svm: &mut LiteSVM,
    teacher: &Keypair,
    program_id: &Pubkey,
    students: Vec<Pubkey>,
) -> Pubkey {
    let list_pda = student_list_pda(program_id, &teacher.pubkey());
    let ix = Instruction::new_with_bytes(
        *program_id,
        &scholarfi_lite_contracts::instruction::CreateStudentList { students }.data(),
        scholarfi_lite_contracts::accounts::CreateStudentList {
            teacher: teacher.pubkey(),
            student_list: list_pda,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    send_ix(svm, teacher, ix).unwrap();
    list_pda
}

fn create_reward_mint(
    svm: &mut LiteSVM,
    teacher: &Keypair,
    program_id: &Pubkey,
    mint_id: u64,
    name: &str,
    symbol: &str,
) -> (Pubkey, Pubkey) {
    let mint = reward_mint_pda(program_id, &teacher.pubkey(), mint_id);
    let config = reward_config_pda(program_id, &teacher.pubkey());
    let ix = Instruction::new_with_bytes(
        *program_id,
        &scholarfi_lite_contracts::instruction::CreateRewardMint {
            name: name.to_string(),
            symbol: symbol.to_string(),
            mint_id,
        }
        .data(),
        scholarfi_lite_contracts::accounts::CreateRewardMint {
            teacher: teacher.pubkey(),
            reward_mint: mint,
            reward_config: config,
            token_program: anchor_spl::token_2022::ID,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    send_ix(svm, teacher, ix).unwrap();
    (mint, config)
}

fn close_reward_mint(
    svm: &mut LiteSVM,
    teacher: &Keypair,
    program_id: &Pubkey,
    mint: Pubkey,
    config: Pubkey,
    mint_id: u64,
) {
    let ix = Instruction::new_with_bytes(
        *program_id,
        &scholarfi_lite_contracts::instruction::CloseRewardMint { mint_id }.data(),
        scholarfi_lite_contracts::accounts::CloseRewardMint {
            teacher: teacher.pubkey(),
            reward_config: config,
            reward_mint: mint,
            token_program: anchor_spl::token_2022::ID,
        }
        .to_account_metas(None),
    );
    send_ix(svm, teacher, ix).unwrap();
}

fn register_achievement(
    svm: &mut LiteSVM,
    teacher: &Keypair,
    program_id: &Pubkey,
    list_pda: Pubkey,
    student: Pubkey,
    achievement_id: u64,
    achievement_type: AchievementType,
) -> Pubkey {
    let achievement = pda(
        program_id,
        &[
            scholarfi_lite_contracts::constants::ACHIEVEMENT_SEED,
            teacher.pubkey().as_ref(),
            student.as_ref(),
            &achievement_id.to_le_bytes(),
        ],
    );
    let ix = Instruction::new_with_bytes(
        *program_id,
        &scholarfi_lite_contracts::instruction::RegisterAchievement {
            achievement_id,
            title: "Completed module".to_string(),
            achievement_type,
            course_id: None,
        }
        .data(),
        scholarfi_lite_contracts::accounts::RegisterAchievement {
            teacher: teacher.pubkey(),
            student,
            student_list: list_pda,
            course: None,
            achievement,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    send_ix(svm, teacher, ix).unwrap();
    achievement
}

fn reward_ix(
    program_id: &Pubkey,
    teacher: &Pubkey,
    student: Pubkey,
    list_pda: Pubkey,
    achievement: Pubkey,
    mint: Pubkey,
    config: Pubkey,
    achievement_id: u64,
    amount: u64,
    mint_id: u64,
) -> Instruction {
    let receipt = pda(
        program_id,
        &[
            scholarfi_lite_contracts::constants::REWARD_RECEIPT_SEED,
            achievement.as_ref(),
        ],
    );
    let ata = get_associated_token_address_with_program_id(
        &student,
        &mint,
        &anchor_spl::token_2022::ID,
    );
    Instruction::new_with_bytes(
        *program_id,
        &scholarfi_lite_contracts::instruction::RewardAchievement {
            achievement_id,
            amount,
            mint_id,
        }
        .data(),
        scholarfi_lite_contracts::accounts::RewardAchievement {
            teacher: *teacher,
            student,
            student_list: list_pda,
            achievement,
            reward_config: config,
            reward_mint: mint,
            reward_receipt: receipt,
            student_token_account: ata,
            token_program: anchor_spl::token_2022::ID,
            associated_token_program: anchor_spl::associated_token::ID,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    )
}

fn token_amount(svm: &LiteSVM, ata: &Pubkey) -> u64 {
    let account = svm.get_account(ata).expect("student ATA");
    let mut data: &[u8] = &account.data;
    anchor_spl::token_interface::TokenAccount::try_deserialize_unchecked(&mut data)
        .unwrap()
        .amount
}

#[test]
fn test_reward_mints_once_into_student_ata() {
    let (mut svm, teacher, program_id) = setup();
    let student = Keypair::new();
    let mint_id = 0u64;
    let list_pda = create_list(&mut svm, &teacher, &program_id, vec![student.pubkey()]);
    let (mint, config) =
        create_reward_mint(&mut svm, &teacher, &program_id, mint_id, "MERIT", "MERIT");

    let stored = RewardConfig::try_deserialize(
        &mut svm.get_account(&config).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(stored.name, "MERIT");
    assert_eq!(stored.symbol, "MERIT");
    assert_eq!(stored.decimals, 6);
    assert_eq!(stored.mint, mint);
    assert_eq!(stored.teacher, teacher.pubkey());

    // Mint is owned by Token-2022.
    let mint_acc = svm.get_account(&mint).unwrap();
    assert_eq!(mint_acc.owner, anchor_spl::token_2022::ID);

    let achievement_id = 1u64;
    let achievement = register_achievement(
        &mut svm,
        &teacher,
        &program_id,
        list_pda,
        student.pubkey(),
        achievement_id,
        AchievementType::Token,
    );

    // 10 whole tokens → 10 * 10^6 base units.
    let amount_base = 10_000_000u64;
    let reward = reward_ix(
        &program_id,
        &teacher.pubkey(),
        student.pubkey(),
        list_pda,
        achievement,
        mint,
        config,
        achievement_id,
        amount_base,
        mint_id,
    );
    assert!(send_ix(&mut svm, &teacher, reward).is_ok());

    let ata = get_associated_token_address_with_program_id(
        &student.pubkey(),
        &mint,
        &anchor_spl::token_2022::ID,
    );
    assert_eq!(token_amount(&svm, &ata), amount_base);

    let receipt_pda = pda(
        &program_id,
        &[
            scholarfi_lite_contracts::constants::REWARD_RECEIPT_SEED,
            achievement.as_ref(),
        ],
    );
    let receipt = RewardReceipt::try_deserialize(
        &mut svm.get_account(&receipt_pda).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(receipt.amount, amount_base);
    assert_eq!(receipt.student, student.pubkey());
    assert_eq!(receipt.mint, mint);

    let again = reward_ix(
        &program_id,
        &teacher.pubkey(),
        student.pubkey(),
        list_pda,
        achievement,
        mint,
        config,
        achievement_id,
        amount_base,
        mint_id,
    );
    assert!(send_ix(&mut svm, &teacher, again).is_err());
    assert_eq!(token_amount(&svm, &ata), amount_base);
}

#[test]
fn test_close_and_recreate_reward_mint() {
    let (mut svm, teacher, program_id) = setup();
    let (mint0, config0) =
        create_reward_mint(&mut svm, &teacher, &program_id, 0, "MERIT", "MERIT");
    assert!(svm.get_account(&config0).is_some());
    assert!(svm.get_account(&mint0).is_some());

    close_reward_mint(&mut svm, &teacher, &program_id, mint0, config0, 0);
    assert!(svm.get_account(&config0).is_none());
    assert!(svm.get_account(&mint0).is_none());

    let (mint1, config1) =
        create_reward_mint(&mut svm, &teacher, &program_id, 1, "MERIT", "MERIT");
    assert_ne!(mint0, mint1);
    let stored = RewardConfig::try_deserialize(
        &mut svm.get_account(&config1).unwrap().data.as_slice(),
    )
    .unwrap();
    assert_eq!(stored.name, "MERIT");
    assert_eq!(stored.mint, mint1);
}

#[test]
fn test_reward_rejects_invalid_awards() {
    let (mut svm, teacher, program_id) = setup();
    let student = Keypair::new();
    let removed = Keypair::new();
    let mint_id = 0u64;
    let list_pda = create_list(
        &mut svm,
        &teacher,
        &program_id,
        vec![student.pubkey(), removed.pubkey()],
    );
    let (mint, config) =
        create_reward_mint(&mut svm, &teacher, &program_id, mint_id, "MERIT", "MERIT");

    let token_achievement = register_achievement(
        &mut svm,
        &teacher,
        &program_id,
        list_pda,
        student.pubkey(),
        1,
        AchievementType::Token,
    );
    let zero = reward_ix(
        &program_id,
        &teacher.pubkey(),
        student.pubkey(),
        list_pda,
        token_achievement,
        mint,
        config,
        1,
        0,
        mint_id,
    );
    assert!(send_ix(&mut svm, &teacher, zero).is_err());

    let nft_achievement = register_achievement(
        &mut svm,
        &teacher,
        &program_id,
        list_pda,
        student.pubkey(),
        2,
        AchievementType::Nft,
    );
    let nft_reward = reward_ix(
        &program_id,
        &teacher.pubkey(),
        student.pubkey(),
        list_pda,
        nft_achievement,
        mint,
        config,
        2,
        5,
        mint_id,
    );
    assert!(send_ix(&mut svm, &teacher, nft_reward).is_err());

    let removed_achievement = register_achievement(
        &mut svm,
        &teacher,
        &program_id,
        list_pda,
        removed.pubkey(),
        3,
        AchievementType::Token,
    );
    let remove = Instruction::new_with_bytes(
        program_id,
        &scholarfi_lite_contracts::instruction::RemoveStudent {
            student: removed.pubkey(),
        }
        .data(),
        scholarfi_lite_contracts::accounts::RemoveStudent {
            teacher: teacher.pubkey(),
            student_list: list_pda,
        }
        .to_account_metas(None),
    );
    send_ix(&mut svm, &teacher, remove).unwrap();

    let not_on_list = reward_ix(
        &program_id,
        &teacher.pubkey(),
        removed.pubkey(),
        list_pda,
        removed_achievement,
        mint,
        config,
        3,
        5,
        mint_id,
    );
    assert!(send_ix(&mut svm, &teacher, not_on_list).is_err());
}
