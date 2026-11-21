use anchor_lang::prelude::*;

use crate::constants::MAX_STUDENTS_PER_LIST;
use crate::error::ErrorCode;
use crate::AddStudent;

pub fn handler(ctx: Context<AddStudent>, student: Pubkey) -> Result<()> {
    let student_list = &mut ctx.accounts.student_list;

    require!(
        !student_list.students.contains(&student),
        ErrorCode::StudentAlreadyOnList
    );
    require!(
        student_list.students.len() < MAX_STUDENTS_PER_LIST,
        ErrorCode::StudentListTooLarge
    );

    student_list.students.push(student);
    msg!("Student {} added to list", student);
    Ok(())
}
