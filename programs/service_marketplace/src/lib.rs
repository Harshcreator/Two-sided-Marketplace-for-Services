use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Transfer};

declare_id!("i6VA27quutNQdauLuZPfHRFCBwXywCEV4JauuswHS2i");

#[program]
pub mod service_marketplace {
    use super::*;

    pub fn list_service(
        ctx: Context<ListService>,
        service_name: String,
        description: String,
        price: u64,
        is_soulbound: bool,
    ) -> Result<()> {
        let service_account = &mut ctx.accounts.service_account;
        service_account.vendor = *ctx.accounts.vendor.key;
        service_account.service_name = service_name;
        service_account.description = description;
        service_account.price = price;
        service_account.is_soulbound = is_soulbound;
        Ok(())
    
    }

    pub fn purchase_service(ctx: Context<PurchaseService>) -> Result<()> {
        let service_account = &ctx.accounts.service_account;
        let vendor = &ctx.accounts.vendor;
        let consumer = &ctx.accounts.consumer;
        let token_program = &ctx.accounts.token_program;

        //transfer funds from consumer to vendor

        let cpi_accounts = Transfer {
            from: consumer.to_account_info(),
            to: vendor.to_account_info(),
            authority: consumer.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, service_account.price)?;

        //Logic for minting Nfts
        Ok(())
    }
}

#[account]
pub struct ServiceAccount {
    pub vendor: Pubkey,
    pub service_name: String,
    pub description: String,
    pub price: u64,
    pub is_soulbound: bool,
}

#[derive(Accounts)]
pub struct ListService<'info> {
    #[account(init, payer = vendor, space = 8 + 8 + 8 + 8 + 1)]
    pub service_account: Account<'info, ServiceAccount>,
    #[account(mut)]
    pub vendor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseService<'info> {
    #[account(mut)]
    pub service_account: Account<'info, ServiceAccount>,
    #[account(mut)]
    pub vendor:Signer<'info>,
    #[account(mut)]
    pub consumer:Signer<'info>,
    #[account(mut)]
    /// CHECK: This is a token account owned by the consumer, no additional checks are needed
    pub consumer_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is a token account owned by the vendor, no additional checks are needed
    pub vendor_token_account: AccountInfo<'info>,
    /// CHECK: This account is the Token Program and is not user-controlled, so no further checks are necessary.
    pub token_program: Program<'info, Token>,

}