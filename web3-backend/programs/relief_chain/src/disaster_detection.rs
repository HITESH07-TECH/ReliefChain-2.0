use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("55J2kUxRUptEk8oshbsrjvmiARUSXumva8P4f34tuSo5");


pub mod disaster_detection_mod {
    use super::*;
    pub fn detect_disaster(ctx: Context<DetectDisaster>, location: String, health_data: String) -> Result<()> {
        // Store user location and health data into the contract's state
        let data = &mut ctx.accounts.user_data;
        data.location = location;
        data.health_data = health_data;

        // Here, you'd interface with the AI system (off-chain) to send this data
        // Placeholder for AI integration: Interface with the AI system for health analysis.
        // AI system can be called off-chain and trigger further contracts/actions if needed.

        Ok(())  
    }
// }

    // Context for the Detect Disaster contract call
    #[derive(Accounts)]
    pub struct DetectDisaster<'info> {
        #[account(init, payer = user, space = 8 + 40 + 256)]  // Initializes a new account to store user data
        pub user_data: Account<'info, UserData>,  // Account to store the user's location and health data
        #[account(mut)]
        pub user: Signer<'info>,  // User signing the transaction (e.g., the affected individual)
        #[account(address = system_program::ID)]
        pub system_program: Program<'info, System>,  // Solana's system program (needed for account creation)
    }

    // Structure for storing user data (location, health data)
    #[account]
    pub struct UserData {
        pub location: String,      // User's location data (e.g., GPS coordinates)
        pub health_data: String,   // User's health metrics (e.g., heart rate, oxygen levels)
    }
}