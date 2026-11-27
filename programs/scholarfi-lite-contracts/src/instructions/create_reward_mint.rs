use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token_interface::{token_metadata_initialize, TokenMetadataInitialize};

use crate::constants::{
    MAX_REWARD_NAME_BYTES, MAX_REWARD_SYMBOL_BYTES, REWARD_DECIMALS, REWARD_MINT_SEED,
};
use crate::error::ErrorCode;
use crate::CreateRewardMint;

/// Extra rent so TokenMetadata can grow the mint after init.
const METADATA_RENT_BUFFER_LAMPORTS: u64 = 10_000_000;

pub fn handler(
    ctx: Context<CreateRewardMint>,
    name: String,
    symbol: String,
    mint_id: u64,
) -> Result<()> {
    require!(!name.is_empty(), ErrorCode::RewardNameEmpty);
    require!(
        name.len() <= MAX_REWARD_NAME_BYTES,
        ErrorCode::RewardNameTooLong
    );
    require!(!symbol.is_empty(), ErrorCode::RewardSymbolEmpty);
    require!(
        symbol.len() <= MAX_REWARD_SYMBOL_BYTES,
        ErrorCode::RewardSymbolTooLong
    );

    // Fund the mint before TokenMetadata reallocs it in place.
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.key(),
            Transfer {
                from: ctx.accounts.teacher.to_account_info(),
                to: ctx.accounts.reward_mint.to_account_info(),
            },
        ),
        METADATA_RENT_BUFFER_LAMPORTS,
    )?;

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

    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            TokenMetadataInitialize {
                program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.reward_mint.to_account_info(),
                metadata: ctx.accounts.reward_mint.to_account_info(),
                mint_authority: ctx.accounts.reward_mint.to_account_info(),
                update_authority: ctx.accounts.reward_mint.to_account_info(),
            },
            &[signer_seeds],
        ),
        name.clone(),
        symbol.clone(),
        String::new(),
    )?;

    let config = &mut ctx.accounts.reward_config;
    config.teacher = ctx.accounts.teacher.key();
    config.mint = ctx.accounts.reward_mint.key();
    config.decimals = REWARD_DECIMALS;
    config.name = name;
    config.symbol = symbol;
    config.bump = ctx.bumps.reward_config;

    msg!(
        "Reward mint {} created (mint_id={})",
        ctx.accounts.reward_mint.key(),
        mint_id
    );
    Ok(())
}
