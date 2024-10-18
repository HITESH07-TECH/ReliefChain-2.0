import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ReliefChain } from "../target/types/relief_chain";

describe("relief_chain", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ReliefChain as Program<ReliefChain>;

  // Initialize required accounts
  let userAccount;

  before(async () => {
    userAccount = anchor.web3.Keypair.generate();
    // Optionally fund or initialize the user account here if needed
  });

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Detects disaster", async () => {
    const location = "Location";
    const healthData = "Heart Rate: 72, Oxygen: 98%";
    
    const tx = await program.methods
      .execDetectDisaster(location, healthData)
      .rpc();
    console.log("Disaster detection transaction signature", tx);
    // Add assertions here
  });

  it("Deploys drones", async () => {
    const criticality = 8;

    const tx = await program.methods
      .execDeployDrones(criticality)
      .rpc();
    console.log("Drone deployment transaction signature", tx);
    // Add assertions here
  });

  it("Requests a resource", async () => {
    const input = "true"; // or "false", depending on what you're testing

    const tx = await program.methods
      .execRequestResource(input)
      .rpc();
    console.log("Resource request transaction signature", tx);
    // Add assertions here
});

  it("Allocates and delivers resources", async () => {
    const resources = "example_resource"; // Replace with actual resource string
    const amount = "100"; // Replace with actual amount string

    const tx = await program.methods
      .execAllocateAndDeliver(resources, amount)
      .rpc();
    
    console.log("Resource allocation and delivery transaction signature", tx);
    
    // Add assertions here
  });

});
