use anchor_lang::prelude::*;

use crate::DeleteAchievement;

pub fn handler(ctx: Context<DeleteAchievement>, _achievement_id: u64) -> Result<()> {
    msg!("Achievement {} deleted", ctx.accounts.achievement.title);
    Ok(())
}
