use anchor_lang::prelude::*;

use crate::constants::MAX_COURSE_TITLE_BYTES;
use crate::error::ErrorCode;
use crate::CreateCourse;

pub fn handler(ctx: Context<CreateCourse>, course_id: u64, title: String) -> Result<()> {
    require!(
        title.len() <= MAX_COURSE_TITLE_BYTES,
        ErrorCode::CourseTitleTooLong
    );

    let course = &mut ctx.accounts.course;
    course.teacher = ctx.accounts.teacher.key();
    course.course_id = course_id;
    course.title = title;
    course.active = true;
    course.bump = ctx.bumps.course;

    msg!("Course {} created", course_id);
    Ok(())
}
