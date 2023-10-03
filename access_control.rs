use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("8oaLSxVgt5WwDL6AypyV52yKKTjqWwcsX6dAiBJBUFa7");

#[program]
pub mod access_control {
    use super::*;
    
    pub fn set_admins(
        ctx: Context<SetAdmins>,
        admins: Vec<Pubkey>
    ) -> Result<()> {
        // Setting owner
        let owner: Pubkey = Pubkey::from_str("<OWNER-ADDRESS>").unwrap();
        let account: &mut Account<Admins> = &mut ctx.accounts.admins;

        require!(
            ctx.accounts.owner.key() == owner,
            Errors::OnlyOwner
        );
        require!(
            admins.len() > 10,
            Errors::MaxAdminSize
        );

        account.admins = admins;
        account.bump = *ctx.bumps.get("admins").unwrap();

        msg!("Admin added.");

        Ok(())
    }

    pub fn add_admin(
        ctx: Context<AddAdmin>,
        admin: Pubkey
    ) -> Result<()> {
        // Setting owner
        let owner: Pubkey = Pubkey::from_str("<OWNER-ADDRESS>").unwrap();
        let account: &mut Account<Admins> = &mut ctx.accounts.admins;

        require!(
            ctx.accounts.owner.key() == owner,
            Errors::OnlyOwner
        );

        let is_exist = account.admins.contains(&admin);
        if is_exist {
            return err!(Errors::AdminExist);
        };

        if account.admins.len() == 10 {
            return err!(Errors::MaxAdminSize);
        };

        account.admins.push(admin);

        msg!("New admin added.");

        Ok(())
    }

    pub fn delete_admin(
        ctx: Context<DeleteAdmin>,
        admin: Pubkey
    ) -> Result<()> {
        // Setting owner
        let owner: Pubkey = Pubkey::from_str("<OWNER-ADDRESS>").unwrap();
        let account: &mut Account<Admins> = &mut ctx.accounts.admins;

        require!(
            ctx.accounts.owner.key() == owner,
            Errors::OnlyOwner
        );

        let is_exist = account.admins.contains(&admin);
        if !is_exist {
            return err!(Errors::AdminNotExist);
        };

        let index = account.admins.iter().position(|pubkey| pubkey == &admin).unwrap();
        account.admins.remove(index);

        msg!("Admin deleted.");

        Ok(())
    }
}

#[derive(Accounts)] 
pub struct SetAdmins<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + (4 + (10 * 32)) + 1,
        seeds = [ b"admins" ],
        bump
    )]
    pub admins: Account<'info, Admins>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AddAdmin<'info> {
    #[account(
        mut,
        seeds = [ b"admins" ],
        bump = admins.bump
    )]
    pub admins: Account<'info, Admins>,
    #[account(mut)]
    pub owner: Signer<'info>
}

#[derive(Accounts)]
pub struct DeleteAdmin<'info> {
    #[account(
        mut,
        seeds = [ b"admins" ],
        bump = admins.bump
    )]
    pub admins: Account<'info, Admins>,
    #[account(mut)]
    pub owner: Signer<'info>
}

#[account]
#[derive(Default)]
pub struct Admins {
    admins: Vec<Pubkey>,
    bump: u8
}

#[error_code]
pub enum Errors {
    #[msg("Only owner!")]
    OnlyOwner,
    #[msg("Max admin size == 10")]
    MaxAdminSize,
    #[msg("Admin not exist")]
    AdminNotExist,
    #[msg("Admin already exist")]
    AdminExist
}