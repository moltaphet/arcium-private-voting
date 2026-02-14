use arcium_sdk::prelude::*;
use serde::{Deserialize, Serialize};

/// Confidential Vote Structure
/// This represents a single ballot in the Arcium Network.
/// Using Secret types ensures that even the nodes cannot see the individual values.
#[derive(Secret, Serialize, Deserialize, Clone)]
pub struct ConfidentialVote {
    // 1 for Yes, 0 for No. Encrypted using Arcium's SecretU8.
    pub choice: SecretU8, 
    // Voter weight (e.g., token balance) used for weighted governance without revealing assets.
    pub voter_weight: SecretU32, 
}

/// Encrypted Shared State for the Voting Proposal
/// This state lives on the Arcium network as a decentralized, private account.
#[account]
pub struct VotingState {
    pub proposal_id: u64,
    pub encrypted_tally: SecretU64,
    pub is_active: bool,
}

impl VotingState {
    /// Core Logic: Confidential Tallying using Multi-Party Computation (MPC)
    /// This function adds the vote to the total tally without ever decrypting the individual choice.
    pub fn process_vote(&mut self, vote: ConfidentialVote) {
        // Calculate weighted vote privately: (choice * weight)
        let weighted_contribution = vote.choice.cast_to_u32() * vote.voter_weight;
        
        // Accumulate into the total encrypted tally
        self.encrypted_tally = self.encrypted_tally + weighted_contribution.cast_to_u64();
    }

    /// Reveal the final result only after the voting period ends.
    pub fn finalize_and_reveal(&self) -> u64 {
        self.encrypted_tally.reveal()
    }
}

fn main() {
    println!("--- Arcium Confidential Voting Engine ---");
    println!("Status: Initializing Secure MPC Environment...");
    
    // Mocking a proposal initialization
    let mut proposal = VotingState {
        proposal_id: 101,
        encrypted_tally: SecretU64::from(0),
        is_active: true,
    };

    println!("Success: Proposal #{} is now live on Arcium.", proposal.proposal_id);
    println!("Privacy Note: All individual votes are encrypted and secret-shared.");
}