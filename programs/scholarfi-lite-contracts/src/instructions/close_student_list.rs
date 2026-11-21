use anchor_lang::prelude::*;

use crate::CloseStudentList;

pub fn handler(ctx: Context<CloseStudentList>) -> Result<()> {
    msg!(
        "Student list closed for teacher {}",
        ctx.accounts.teacher.key()
    );
    Ok(())
}
