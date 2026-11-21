use anchor_lang::prelude::*;

use crate::CloseCourse;

pub fn handler(ctx: Context<CloseCourse>, course_id: u64) -> Result<()> {
    msg!(
        "Course {} closed by teacher {}",
        course_id,
        ctx.accounts.teacher.key()
    );
    Ok(())
}
