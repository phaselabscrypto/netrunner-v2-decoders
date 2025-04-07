



use super::JitoTipDistributionDecoder;
pub mod initialize;
pub mod initialize_tip_distribution_account;
pub mod update_config;
pub mod upload_merkle_root;
pub mod close_claim_status;
pub mod close_tip_distribution_account;
pub mod claim;
pub mod tip_distribution_account_initialized_event;
pub mod validator_commission_bps_updated_event;
pub mod merkle_root_upload_authority_updated_event;
pub mod config_updated_event;
pub mod claimed_event;
pub mod merkle_root_uploaded_event;
pub mod tip_distribution_account_closed_event;
pub mod claim_status_closed_event;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum JitoTipDistributionInstruction {
    Initialize(initialize::Initialize),
    InitializeTipDistributionAccount(initialize_tip_distribution_account::InitializeTipDistributionAccount),
    UpdateConfig(update_config::UpdateConfig),
    UploadMerkleRoot(upload_merkle_root::UploadMerkleRoot),
    CloseClaimStatus(close_claim_status::CloseClaimStatus),
    CloseTipDistributionAccount(close_tip_distribution_account::CloseTipDistributionAccount),
    Claim(claim::Claim),
    TipDistributionAccountInitializedEvent(tip_distribution_account_initialized_event::TipDistributionAccountInitializedEvent),
    ValidatorCommissionBpsUpdatedEvent(validator_commission_bps_updated_event::ValidatorCommissionBpsUpdatedEvent),
    MerkleRootUploadAuthorityUpdatedEvent(merkle_root_upload_authority_updated_event::MerkleRootUploadAuthorityUpdatedEvent),
    ConfigUpdatedEvent(config_updated_event::ConfigUpdatedEvent),
    ClaimedEvent(claimed_event::ClaimedEvent),
    MerkleRootUploadedEvent(merkle_root_uploaded_event::MerkleRootUploadedEvent),
    TipDistributionAccountClosedEvent(tip_distribution_account_closed_event::TipDistributionAccountClosedEvent),
    ClaimStatusClosedEvent(claim_status_closed_event::ClaimStatusClosedEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for JitoTipDistributionDecoder {
    type InstructionType = JitoTipDistributionInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JitoTipDistributionInstruction::Initialize => initialize::Initialize,
            JitoTipDistributionInstruction::InitializeTipDistributionAccount => initialize_tip_distribution_account::InitializeTipDistributionAccount,
            JitoTipDistributionInstruction::UpdateConfig => update_config::UpdateConfig,
            JitoTipDistributionInstruction::UploadMerkleRoot => upload_merkle_root::UploadMerkleRoot,
            JitoTipDistributionInstruction::CloseClaimStatus => close_claim_status::CloseClaimStatus,
            JitoTipDistributionInstruction::CloseTipDistributionAccount => close_tip_distribution_account::CloseTipDistributionAccount,
            JitoTipDistributionInstruction::Claim => claim::Claim,
            JitoTipDistributionInstruction::TipDistributionAccountInitializedEvent => tip_distribution_account_initialized_event::TipDistributionAccountInitializedEvent,
            JitoTipDistributionInstruction::ValidatorCommissionBpsUpdatedEvent => validator_commission_bps_updated_event::ValidatorCommissionBpsUpdatedEvent,
            JitoTipDistributionInstruction::MerkleRootUploadAuthorityUpdatedEvent => merkle_root_upload_authority_updated_event::MerkleRootUploadAuthorityUpdatedEvent,
            JitoTipDistributionInstruction::ConfigUpdatedEvent => config_updated_event::ConfigUpdatedEvent,
            JitoTipDistributionInstruction::ClaimedEvent => claimed_event::ClaimedEvent,
            JitoTipDistributionInstruction::MerkleRootUploadedEvent => merkle_root_uploaded_event::MerkleRootUploadedEvent,
            JitoTipDistributionInstruction::TipDistributionAccountClosedEvent => tip_distribution_account_closed_event::TipDistributionAccountClosedEvent,
            JitoTipDistributionInstruction::ClaimStatusClosedEvent => claim_status_closed_event::ClaimStatusClosedEvent,
        )
    }
}