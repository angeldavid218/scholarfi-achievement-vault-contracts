use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, CloseAccount};

use crate::constants::REWARD_MINT_SEED;
use crate::error::ErrorCode;
use crate::CloseRewardMint;

pub fn handler(ctx: Context<CloseRewardMint>, mint_id: u64) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.reward_config.mint,
        ctx.accounts.reward_mint.key(),
        ErrorCode::RewardMintMismatch
    );

    let mint_info = ctx.accounts.reward_mint.to_account_info();
    let token_2022_id = anchor_spl::token_2022::ID;

    // Classic SPL mints cannot be closed — leave them orphaned.
    // Token-2022 mints with MintCloseAuthority can be closed when supply is 0.
    if *mint_info.owner == token_2022_id {
        let data = mint_info.try_borrow_data()?;
        // Mint layout: mint_authority (36) + supply (u64 LE at offset 36).
        require!(data.len() >= 44, ErrorCode::RewardMintMismatch);
        let supply = u64::from_le_bytes(data[36..44].try_into().unwrap());
        require!(supply == 0, ErrorCode::RewardMintSupplyNonZero);
        drop(data);

        let (expected_mint, bump) = Pubkey::find_program_address(
            &[
                REWARD_MINT_SEED,
                ctx.accounts.teacher.key().as_ref(),
                &mint_id.to_le_bytes(),
            ],
            ctx.program_id,
        );
        require_keys_eq!(
            expected_mint,
            ctx.accounts.reward_mint.key(),
            ErrorCode::RewardMintIdMismatch
        );

        let teacher_key = ctx.accounts.teacher.key();
        let mint_id_bytes = mint_id.to_le_bytes();
        let bump_seed = [bump];
        let signer_seeds: &[&[u8]] = &[
            REWARD_MINT_SEED,
            teacher_key.as_ref(),
            &mint_id_bytes,
            &bump_seed,
        ];

        token_interface::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            CloseAccount {
                account: ctx.accounts.reward_mint.to_account_info(),
                destination: ctx.accounts.teacher.to_account_info(),
                authority: ctx.accounts.reward_mint.to_account_info(),
            },
            &[signer_seeds],
        ))?;
    }

    msg!(
        "Reward config closed for mint {} (mint_id={})",
        ctx.accounts.reward_mint.key(),
        mint_id
    );
    Ok(())
}
