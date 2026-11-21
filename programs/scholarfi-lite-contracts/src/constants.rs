use anchor_lang::prelude::*;

pub const STUDENT_LIST_SEED: &[u8] = b"student_list";
pub const ACHIEVEMENT_SEED: &[u8] = b"achievement";
pub const COURSE_SEED: &[u8] = b"course";

/// Max UTF-8 byte length for achievement titles.
pub const MAX_TITLE_BYTES: usize = 128;

/// Max UTF-8 byte length for course titles.
pub const MAX_COURSE_TITLE_BYTES: usize = 64;

/// Must match `#[max_len(...)]` on `StudentList::students`.
pub const MAX_STUDENTS_PER_LIST: usize = 100;
