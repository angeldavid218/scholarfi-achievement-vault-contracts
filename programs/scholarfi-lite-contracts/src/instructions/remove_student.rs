use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::RemoveStudent;

pub fn handler(ctx: Context<RemoveStudent>, student: Pubkey) -> Result<()> {
    let student_list = &mut ctx.accounts.student_list;

    let Some(index) = student_list.students.iter().position(|s| s == &student) else {
        return err!(ErrorCode::StudentNotFound);
    };

    student_list.students.remove(index);
    msg!("Student {} removed from list", student);
    Ok(())
}
