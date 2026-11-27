use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid achievement id")]
    InvalidAchievementId,
    #[msg("Student is not on this teacher's student list")]
    StudentNotOnList,
    #[msg("Achievement title exceeds max length")]
    TitleTooLong,
    #[msg("Student list exceeds max allowed students")]
    StudentListTooLarge,
    #[msg("Student list account does not belong to this teacher")]
    UnauthorizedTeacher,
    #[msg("Student is already on this teacher's student list")]
    StudentAlreadyOnList,
    #[msg("Student was not found on this teacher's student list")]
    StudentNotFound,
    #[msg("Course title exceeds max length")]
    CourseTitleTooLong,
    #[msg("Course account does not belong to this teacher")]
    UnauthorizedCourseTeacher,
    #[msg("Course account must be provided when linking a course_id")]
    CourseRequired,
    #[msg("Provided course does not match the given course_id")]
    CourseIdMismatch,
    #[msg("Reward token name is empty")]
    RewardNameEmpty,
    #[msg("Reward token name exceeds max length")]
    RewardNameTooLong,
    #[msg("Reward token symbol is empty")]
    RewardSymbolEmpty,
    #[msg("Reward token symbol exceeds max length")]
    RewardSymbolTooLong,
    #[msg("Reward amount must be between 1 and the maximum")]
    InvalidRewardAmount,
    #[msg("Only token achievements can be rewarded")]
    AchievementNotToken,
    #[msg("Achievement does not belong to this student")]
    AchievementStudentMismatch,
    #[msg("Reward mint does not match the teacher's reward config")]
    RewardMintMismatch,
    #[msg("Token program must be the Token-2022 program")]
    InvalidTokenProgram,
    #[msg("Reward mint cannot be closed while supply is non-zero")]
    RewardMintSupplyNonZero,
    #[msg("Reward mint does not match the provided mint_id PDA")]
    RewardMintIdMismatch,
}
