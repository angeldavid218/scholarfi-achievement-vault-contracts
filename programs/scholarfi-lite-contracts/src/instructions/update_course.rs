use anchor_lang::prelude::*;

use crate::constants::MAX_COURSE_TITLE_BYTES;
use crate::error::ErrorCode;
use crate::UpdateCourse;

pub fn handler(
    ctx: Context<UpdateCourse>,
    course_id: u64,
    title: String,
    active: bool,
) -> Result<()> {
    require!(
        title.len() <= MAX_COURSE_TITLE_BYTES,
        ErrorCode::CourseTitleTooLong
    );

    let course = &mut ctx.accounts.course;
    course.title = title;
    course.active = active;

    msg!("Course {} updated", course_id);
    Ok(())
}
