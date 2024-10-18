use anchor_lang::prelude::*;
declare_id!("55J2kUxRUptEk8oshbsrjvmiARUSXumva8P4f34tuSo5");

pub mod resource_allocation_mod {
    use super::*;
    // use tokio::time::{sleep, Duration};

    pub fn allocate_and_deliver(ctx: Context<AllocateAndDeliver>, resources: String, amount: String) -> Result<()> {
        // Log the start of resource allocation
        msg!("Allocating resources, {}, from supplier...", resources);
        // sleep(Duration::from_secs(1.5)).await;

        // Log that resource delivery has been initiated
        msg!("Resource, {}, delivery initiated.", resources);
        // sleep(Duration::from_secs(1.5)).await;

        // Log the intention to release payment to the supplier
        msg!("Releasing payment of {} to supplier.", amount);
        // sleep(Duration::from_secs(1.5)).await;

        // Log that the resource has been delivered
        msg!("Resource, {}, delivered...", resources);
        // sleep(Duration::from_secs(1.5)).await;

        // Log confirmation of payment received
        msg!("Payment of {} received", amount);
        
        Ok(())
    }

    // Context for the Allocate and Deliver contract
    #[derive(Accounts)]
    pub struct AllocateAndDeliver<'info> {
        #[account(mut)]
        pub user: Signer<'info>,  // User signing the transaction (confirmation of delivery)
    }
}
