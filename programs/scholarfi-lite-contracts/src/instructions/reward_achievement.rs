use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, MintToChecked};

use crate::constants::{MAX_REWARD_AMOUNT, REWARD_MINT_SEED};
use crate::error::ErrorCode;
use crate::state::AchievementType;
use crate::RewardAchievement;

pub fn handler(
    ctx: Context<RewardAchievement>,
    _achievement_id: u64,
    amount: u64,
    mint_id: u64,
) -> Result<()> {
    require!(
        (1..=MAX_REWARD_AMOUNT).contains(&amount),
        ErrorCode::InvalidRewardAmount
    );
    require!(
        ctx.accounts.achievement.achievement_type == AchievementType::Token,
        ErrorCode::AchievementNotToken
    );
    require!(
        ctx.accounts
            .student_list
            .students
            .contains(&ctx.accounts.student.key()),
        ErrorCode::StudentNotOnList
    );

    let teacher_key = ctx.accounts.teacher.key();
    let mint_id_bytes = mint_id.to_le_bytes();
    let bump = ctx.bumps.reward_mint;
    let bump_seed = [bump];
    let signer_seeds: &[&[u8]] = &[
        REWARD_MINT_SEED,
        teacher_key.as_ref(),
        &mint_id_bytes,
        &bump_seed,
    ];

    let decimals = ctx.accounts.reward_mint.decimals;
    token_interface::mint_to_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            MintToChecked {
                mint: ctx.accounts.reward_mint.to_account_info(),
                to: ctx.accounts.student_token_account.to_account_info(),
                authority: ctx.accounts.reward_mint.to_account_info(),
            },
            &[signer_seeds],
        ),
        amount,
        decimals,
    )?;

    let receipt = &mut ctx.accounts.reward_receipt;
    receipt.achievement = ctx.accounts.achievement.key();
    receipt.student = ctx.accounts.student.key();
    receipt.mint = ctx.accounts.reward_mint.key();
    receipt.amount = amount;
    receipt.bump = ctx.bumps.reward_receipt;

    msg!(
        "Rewarded {} tokens to {}",
        amount,
        ctx.accounts.student.key()
    );
    Ok(())
}
