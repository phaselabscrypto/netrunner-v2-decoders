use super::JupiterMerkleDistributorDecoder;
pub mod claim_locked;
pub mod claim_locked_and_stake;
pub mod claimed_event;
pub mod clawback;
pub mod close_claim_status;
pub mod close_distributor;
pub mod new_claim;
pub mod new_claim_and_stake;
pub mod new_claim_event;
pub mod new_distributor;
pub mod set_activation_point;
pub mod set_admin;
pub mod set_clawback_receiver;
pub mod set_operator;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum JupiterMerkleDistributorInstruction {
    NewDistributor(new_distributor::NewDistributor),
    CloseDistributor(close_distributor::CloseDistributor),
    CloseClaimStatus(close_claim_status::CloseClaimStatus),
    SetActivationPoint(set_activation_point::SetActivationPoint),
    Clawback(clawback::Clawback),
    SetClawbackReceiver(set_clawback_receiver::SetClawbackReceiver),
    SetAdmin(set_admin::SetAdmin),
    SetOperator(set_operator::SetOperator),
    NewClaim(new_claim::NewClaim),
    ClaimLocked(claim_locked::ClaimLocked),
    NewClaimAndStake(new_claim_and_stake::NewClaimAndStake),
    ClaimLockedAndStake(claim_locked_and_stake::ClaimLockedAndStake),
    NewClaimEvent(new_claim_event::NewClaimEvent),
    ClaimedEvent(claimed_event::ClaimedEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterMerkleDistributorDecoder {
    type InstructionType = JupiterMerkleDistributorInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterMerkleDistributorInstruction::NewDistributor => new_distributor::NewDistributor,
            JupiterMerkleDistributorInstruction::CloseDistributor => close_distributor::CloseDistributor,
            JupiterMerkleDistributorInstruction::CloseClaimStatus => close_claim_status::CloseClaimStatus,
            JupiterMerkleDistributorInstruction::SetActivationPoint => set_activation_point::SetActivationPoint,
            JupiterMerkleDistributorInstruction::Clawback => clawback::Clawback,
            JupiterMerkleDistributorInstruction::SetClawbackReceiver => set_clawback_receiver::SetClawbackReceiver,
            JupiterMerkleDistributorInstruction::SetAdmin => set_admin::SetAdmin,
            JupiterMerkleDistributorInstruction::SetOperator => set_operator::SetOperator,
            JupiterMerkleDistributorInstruction::NewClaim => new_claim::NewClaim,
            JupiterMerkleDistributorInstruction::ClaimLocked => claim_locked::ClaimLocked,
            JupiterMerkleDistributorInstruction::NewClaimAndStake => new_claim_and_stake::NewClaimAndStake,
            JupiterMerkleDistributorInstruction::ClaimLockedAndStake => claim_locked_and_stake::ClaimLockedAndStake,
            JupiterMerkleDistributorInstruction::NewClaimEvent => new_claim_event::NewClaimEvent,
            JupiterMerkleDistributorInstruction::ClaimedEvent => claimed_event::ClaimedEvent,
        )
    }
}
