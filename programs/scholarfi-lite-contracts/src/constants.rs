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

pub const REWARD_MINT_SEED: &[u8] = b"reward_mint";
pub const REWARD_CONFIG_SEED: &[u8] = b"reward_config";
pub const REWARD_RECEIPT_SEED: &[u8] = b"reward";

/// Display decimals for classroom reward mints (Explorer treats 0 as NFT-like).
pub const REWARD_DECIMALS: u8 = 6;

/// Must match `#[max_len(...)]` on `RewardConfig::name`.
pub const MAX_REWARD_NAME_BYTES: usize = 32;

/// Must match `#[max_len(...)]` on `RewardConfig::symbol`.
pub const MAX_REWARD_SYMBOL_BYTES: usize = 10;

/// Inclusive upper bound for a single achievement reward, in base units
/// (1e9 whole tokens × 10^REWARD_DECIMALS).
pub const MAX_REWARD_AMOUNT: u64 = 1_000_000_000_000_000;
