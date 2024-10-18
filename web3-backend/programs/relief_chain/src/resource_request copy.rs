use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("55J2kUxRUptEk8oshbsrjvmiARUSXumva8P4f34tuSo5");


pub mod resource_request_mod {
    use super::*;
    pub fn request_resource(ctx: Context<RequestResource>, amount: u64, resource: String) -> Result<()> {
        // Store the payment in the escrow account
        let escrow = &mut ctx.accounts.escrow;
        escrow.amount = amount;
        escrow.resource = resource.clone();

        msg!("Resource requested: {}", resource);
        msg!("Payment of {} placed in escrow.", amount);

        Ok(())
    }
// }

// Context for the Request Resource contract
    #[derive(Accounts)]
    pub struct RequestResource<'info> {
        #[account(init, payer = user, space = 8 + 8 + 64)]  // Creates a new escrow account to store payment and resource request
        pub escrow: Account<'info, EscrowAccount>,  // The escrow account holding the user's payment
        #[account(mut)]
        pub user: Signer<'info>,  // User making the resource request
        #[account(address = system_program::ID)]
        pub system_program: Program<'info, System>,  // Solana system program (for account creation)
    }

    // Structure for the Escrow Account
    #[account]
    pub struct EscrowAccount {
        pub amount: u64,         // The payment amount
        pub resource: String,    // The resource being requested
    }
}

1. msg!("Allocating resources, {}, from supplier...", resources);

2. msg!("Resource, {}, delivery initiated.", resources);

3. msg!("Releasing payment of {} to supplier.", amount);

4. msg!("Resource, {}, delivered...", resources);

5. msg!("Payment of {} received",amount);

