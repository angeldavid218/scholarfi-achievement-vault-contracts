pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use state::*;

declare_id!("7JTpTv6ftwRJwyXasGjU7L1swFnrizKh7nEeHnSbVMbs");

#[program]
pub mod scholarfi_lite_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn create_student_list(
        ctx: Context<CreateStudentList>,
        students: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::create_student_list::handler(ctx, students)
    }

    pub fn add_student(ctx: Context<AddStudent>, student: Pubkey) -> Result<()> {
        instructions::add_student::handler(ctx, student)
    }

    pub fn remove_student(ctx: Context<RemoveStudent>, student: Pubkey) -> Result<()> {
        instructions::remove_student::handler(ctx, student)
    }

    pub fn close_student_list(ctx: Context<CloseStudentList>) -> Result<()> {
        instructions::close_student_list::handler(ctx)
    }

    pub fn create_course(ctx: Context<CreateCourse>, course_id: u64, title: String) -> Result<()> {
        instructions::create_course::handler(ctx, course_id, title)
    }

    pub fn update_course(
        ctx: Context<UpdateCourse>,
        course_id: u64,
        title: String,
        active: bool,
    ) -> Result<()> {
        instructions::update_course::handler(ctx, course_id, title, active)
    }

    pub fn close_course(ctx: Context<CloseCourse>, course_id: u64) -> Result<()> {
        instructions::close_course::handler(ctx, course_id)
    }

    pub fn register_achievement(
        ctx: Context<RegisterAchievement>,
        achievement_id: u64,
        title: String,
        achievement_type: AchievementType,
        course_id: Option<u64>,
    ) -> Result<()> {
        instructions::register_achievement::handler(
            ctx,
            achievement_id,
            title,
            achievement_type,
            course_id,
        )
    }

    pub fn delete_achievement(
        ctx: Context<DeleteAchievement>,
        achievement_id: u64,
    ) -> Result<()> {
        instructions::delete_achievement::handler(ctx, achievement_id)
    }

    pub fn update_achievement(
        ctx: Context<UpdateAchievement>,
        achievement_id: u64,
        title: String,
        achievement_type: AchievementType,
        course_id: Option<u64>,
    ) -> Result<()> {
        instructions::update_achievement::handler(
            ctx,
            achievement_id,
            title,
            achievement_type,
            course_id,
        )
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateStudentList<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    #[account(
        init,
        payer = teacher,
        space = 8 + StudentList::INIT_SPACE,
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump
    )]
    pub student_list: Account<'info, StudentList>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddStudent<'info> {
    pub teacher: Signer<'info>,
    #[account(
        mut,
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump = student_list.bump,
        constraint = student_list.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher
    )]
    pub student_list: Account<'info, StudentList>,
}

#[derive(Accounts)]
pub struct RemoveStudent<'info> {
    pub teacher: Signer<'info>,
    #[account(
        mut,
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump = student_list.bump,
        constraint = student_list.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher
    )]
    pub student_list: Account<'info, StudentList>,
}

#[derive(Accounts)]
pub struct CloseStudentList<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    #[account(
        mut,
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump = student_list.bump,
        constraint = student_list.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher,
        close = teacher
    )]
    pub student_list: Account<'info, StudentList>,
}

#[derive(Accounts)]
#[instruction(course_id: u64)]
pub struct CreateCourse<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    #[account(
        init,
        payer = teacher,
        space = 8 + Course::INIT_SPACE,
        seeds = [
            crate::constants::COURSE_SEED,
            teacher.key().as_ref(),
            &course_id.to_le_bytes(),
        ],
        bump
    )]
    pub course: Account<'info, Course>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(course_id: u64)]
pub struct UpdateCourse<'info> {
    pub teacher: Signer<'info>,
    #[account(
        mut,
        seeds = [
            crate::constants::COURSE_SEED,
            teacher.key().as_ref(),
            &course_id.to_le_bytes(),
        ],
        bump = course.bump,
        constraint = course.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedCourseTeacher
    )]
    pub course: Account<'info, Course>,
}

#[derive(Accounts)]
#[instruction(course_id: u64)]
pub struct CloseCourse<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    #[account(
        mut,
        seeds = [
            crate::constants::COURSE_SEED,
            teacher.key().as_ref(),
            &course_id.to_le_bytes(),
        ],
        bump = course.bump,
        constraint = course.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedCourseTeacher,
        close = teacher
    )]
    pub course: Account<'info, Course>,
}

#[derive(Accounts)]
#[instruction(achievement_id: u64, title: String, achievement_type: AchievementType, course_id: Option<u64>)]
pub struct RegisterAchievement<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    /// CHECK: only used as a public key; membership is enforced against the student list.
    pub student: UncheckedAccount<'info>,
    #[account(
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump = student_list.bump,
        constraint = student_list.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher
    )]
    pub student_list: Account<'info, StudentList>,
    /// Optional Course PDA. Required when `course_id` is Some.
    pub course: Option<Account<'info, Course>>,
    #[account(
        init,
        payer = teacher,
        space = 8 + Achievement::INIT_SPACE,
        seeds = [
            crate::constants::ACHIEVEMENT_SEED,
            teacher.key().as_ref(),
            student.key().as_ref(),
            &achievement_id.to_le_bytes(),
        ],
        bump
    )]
    pub achievement: Account<'info, Achievement>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(achievement_id: u64)]
pub struct DeleteAchievement<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    /// CHECK: only used as a public key; membership is enforced against the student list.
    pub student: UncheckedAccount<'info>,
    #[account(
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump = student_list.bump,
        constraint = student_list.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher
    )]
    pub student_list: Account<'info, StudentList>,
    #[account(
        mut,
        seeds = [
            crate::constants::ACHIEVEMENT_SEED,
            teacher.key().as_ref(),
            student.key().as_ref(),
            &achievement_id.to_le_bytes(),
        ],
        bump = achievement.bump,
        constraint = achievement.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher,
        close = teacher
    )]
    pub achievement: Account<'info, Achievement>,
}

#[derive(Accounts)]
#[instruction(achievement_id: u64, title: String, achievement_type: AchievementType, course_id: Option<u64>)]
pub struct UpdateAchievement<'info> {
    #[account(mut)]
    pub teacher: Signer<'info>,
    /// CHECK: only used as a public key; membership is enforced against the student list.
    pub student: UncheckedAccount<'info>,
    #[account(
        seeds = [crate::constants::STUDENT_LIST_SEED, teacher.key().as_ref()],
        bump = student_list.bump,
        constraint = student_list.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher
    )]
    pub student_list: Account<'info, StudentList>,
    /// Optional Course PDA. Required when `course_id` is Some.
    pub course: Option<Account<'info, Course>>,
    #[account(
        mut,
        seeds = [
            crate::constants::ACHIEVEMENT_SEED,
            teacher.key().as_ref(),
            student.key().as_ref(),
            &achievement_id.to_le_bytes(),
        ],
        bump = achievement.bump,
        constraint = achievement.teacher == teacher.key() @ crate::error::ErrorCode::UnauthorizedTeacher,
    )]
    pub achievement: Account<'info, Achievement>,
}
