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
}
