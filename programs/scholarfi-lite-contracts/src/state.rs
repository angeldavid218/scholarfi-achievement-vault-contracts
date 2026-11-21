use anchor_lang::prelude::*;

/// How the achievement is represented off-chain or in a follow-up mint flow.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum AchievementType {
    Token,
    Nft,
}

/// One issued achievement for a student, signed by the teacher.
#[account]
#[derive(InitSpace)]
pub struct Achievement {
    pub student: Pubkey,
    pub teacher: Pubkey,
    #[max_len(128)]
    pub title: String,
    pub achievement_type: AchievementType,
    /// Optional link to a teacher-owned Course PDA.
    pub course_id: Option<u64>,
    pub bump: u8,
}

/// Teacher-scoped list of student wallets allowed to receive achievements.
#[account]
#[derive(InitSpace)]
pub struct StudentList {
    pub teacher: Pubkey,
    pub bump: u8,
    #[max_len(100)]
    pub students: Vec<Pubkey>,
}

/// Teacher-owned course that achievements can optionally reference.
#[account]
#[derive(InitSpace)]
pub struct Course {
    pub teacher: Pubkey,
    pub course_id: u64,
    #[max_len(64)]
    pub title: String,
    pub active: bool,
    pub bump: u8,
}
