use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_world_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_messages = 0;
        Ok(())
    }

    pub fn send_message(ctx: Context<SendMessage>, message: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let timestamp = Clock::get().unwrap().unix_timestamp;

        let message_struct = MessageStruct {
            sender: *user.to_account_info().key,
            message: message.to_string(),
            timestamp,
        };

        base_account.message_list.push(message_struct);
        base_account.total_messages += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MessageStruct {
    pub sender: Pubkey,
    pub message: String,
    pub timestamp: i64,
}

#[account]
pub struct BaseAccount {
    pub total_messages: u64,
    pub message_list: Vec<MessageStruct>,
}
