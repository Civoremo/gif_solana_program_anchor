use anchor_lang::prelude::*;

declare_id!("wCgaXxvL5Tnnp7DncT6WFgZRCHSZGhvD5QhQm6BYxc7");

#[program]
pub mod gif_solana_program {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            // user_address: *base_account.to_account_info().key,
            user_address: *user.to_account_info().key,
            votes: 0
        };

        base_account.total_gifs += 1;
        base_account.gif_list.push(item);
        
        Ok(())
    }

    pub fn upvote_gif(ctx: Context<AddVote>, gif_id: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        base_account.gif_list[gif_id as usize].votes += 1;

        Ok(())
    }

    pub fn tip_user(ctx: Context<SendTip>,  amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from_user.key(),
            &ctx.accounts.to_user.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from_user.to_account_info(),
                ctx.accounts.to_user.to_account_info(),
            ],
        )?;
        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u64
}

#[derive(Accounts)]
pub struct AddVote<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct SendTip<'info> {
    #[account(mut)]
    pub from_user: Signer<'info>,
    #[account(mut)]
    pub to_user: AccountInfo<'info>,
    system_program: Program<'info, System>
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>
}
