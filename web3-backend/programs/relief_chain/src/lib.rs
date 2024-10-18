// lib.rs
use anchor_lang::prelude::*;
use crate::disaster_detection::disaster_detection_mod::*;
use crate::drone_deployment::drone_deployment_mod::*;
use crate::resource_request::resource_request_mod::*;
use crate::resource_allocation::resource_allocation_mod::*;

declare_id!("55J2kUxRUptEk8oshbsrjvmiARUSXumva8P4f34tuSo5");

pub mod disaster_detection;
pub mod drone_deployment;
pub mod resource_request;
pub mod resource_allocation;

#[program]
pub mod relief_chain {
    use super::*;

    // Initialize function (optional, for program initialization)
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // Function to detect disaster
    pub fn exec_detect_disaster(ctx: Context<DetectDisaster>, location: String, health_data: String) -> Result<()> {
        detect_disaster(ctx, location, health_data)
    }
    

    // Function to deploy drones
    pub fn exec_deploy_drones(ctx: Context<DeployDrones>, criticality: u8) -> Result<()> {
        deploy_drones(ctx, criticality)
    }

    // Function to request a resource
    pub fn exec_request_resource(ctx: Context<RequestResource>, input: String) -> Result<bool> {
        request_resource(ctx, input) // Pass the string input
    }    

    // Function to allocate and deliver resources
    pub fn exec_allocate_and_deliver(ctx: Context<AllocateAndDeliver>, resources: String, amount: String) -> Result<()> {
        allocate_and_deliver(ctx, resources, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
