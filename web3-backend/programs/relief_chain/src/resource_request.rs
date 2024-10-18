use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;


declare_id!("55J2kUxRUptEk8oshbsrjvmiARUSXumva8P4f34tuSo5");

pub mod resource_request_mod {
    use super::*;

    pub fn request_resource(ctx: Context<RequestResource>, input: String) -> Result<bool> {
        // Convert the input string to boolean
        let result = match input.to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => false
        };

        msg!("Input string '{}' converted to boolean: {}", input, result);
        Ok(result)
    }

    // Context for the Request Resource contract
    #[derive(Accounts)]
    pub struct RequestResource<'info> {
        #[account(mut)]
        pub user: Signer<'info>,  // User making the resource request
        #[account(address = system_program::ID)]
        pub system_program: Program<'info, System>,  // Solana system program (for account creation)
    }

}
