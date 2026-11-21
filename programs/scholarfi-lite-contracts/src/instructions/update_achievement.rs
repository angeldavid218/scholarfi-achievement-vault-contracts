use anchor_lang::prelude::*;

use crate::constants::MAX_TITLE_BYTES;
use crate::error::ErrorCode;
use crate::state::AchievementType;
use crate::UpdateAchievement;

pub fn handler(
    ctx: Context<UpdateAchievement>,
    achievement_id: u64,
    title: String,
    achievement_type: AchievementType,
    course_id: Option<u64>,
) -> Result<()> {
    require!(title.len() <= MAX_TITLE_BYTES, ErrorCode::TitleTooLong);

    let linked_course_id = match course_id {
        Some(id) => {
            let course = ctx
                .accounts
                .course
                .as_ref()
                .ok_or_else(|| error!(ErrorCode::CourseRequired))?;
            require!(course.course_id == id, ErrorCode::CourseIdMismatch);
            require!(
                course.teacher == ctx.accounts.teacher.key(),
                ErrorCode::UnauthorizedCourseTeacher
            );
            Some(id)
        }
        None => {
            require!(ctx.accounts.course.is_none(), ErrorCode::CourseIdMismatch);
            None
        }
    };

    let achievement = &mut ctx.accounts.achievement;
    achievement.title = title;
    achievement.achievement_type = achievement_type;
    achievement.course_id = linked_course_id;

    msg!("Achievement {} updated successfully", achievement_id);
    Ok(())
}
