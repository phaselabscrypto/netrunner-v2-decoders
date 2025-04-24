use super::ValidatorBondsDecoder;
pub mod cancel_settlement;
pub mod cancel_settlement_event;
pub mod cancel_withdraw_request;
pub mod cancel_withdraw_request_event;
pub mod claim_settlement_event;
pub mod claim_settlement_v2;
pub mod claim_settlement_v2_event;
pub mod claim_withdraw_request;
pub mod claim_withdraw_request_event;
pub mod close_settlement_event;
pub mod close_settlement_v2;
pub mod configure_bond;
pub mod configure_bond_event;
pub mod configure_bond_with_mint;
pub mod configure_bond_with_mint_event;
pub mod configure_config;
pub mod configure_config_event;
pub mod emergency_pause;
pub mod emergency_pause_event;
pub mod emergency_resume;
pub mod emergency_resume_event;
pub mod fund_bond;
pub mod fund_bond_event;
pub mod fund_settlement;
pub mod fund_settlement_event;
pub mod init_bond;
pub mod init_bond_event;
pub mod init_config;
pub mod init_config_event;
pub mod init_settlement;
pub mod init_settlement_event;
pub mod init_withdraw_request;
pub mod init_withdraw_request_event;
pub mod merge_stake;
pub mod merge_stake_event;
pub mod mint_bond;
pub mod mint_bond_event;
pub mod reset_stake;
pub mod reset_stake_event;
pub mod upsize_settlement_claims;
pub mod withdraw_stake;
pub mod withdraw_stake_event;

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
pub enum ValidatorBondsInstruction {
    InitConfig(init_config::InitConfig),
    ConfigureConfig(configure_config::ConfigureConfig),
    InitBond(init_bond::InitBond),
    ConfigureBond(configure_bond::ConfigureBond),
    ConfigureBondWithMint(configure_bond_with_mint::ConfigureBondWithMint),
    MintBond(mint_bond::MintBond),
    FundBond(fund_bond::FundBond),
    InitWithdrawRequest(init_withdraw_request::InitWithdrawRequest),
    CancelWithdrawRequest(cancel_withdraw_request::CancelWithdrawRequest),
    ClaimWithdrawRequest(claim_withdraw_request::ClaimWithdrawRequest),
    InitSettlement(init_settlement::InitSettlement),
    UpsizeSettlementClaims(upsize_settlement_claims::UpsizeSettlementClaims),
    CancelSettlement(cancel_settlement::CancelSettlement),
    FundSettlement(fund_settlement::FundSettlement),
    MergeStake(merge_stake::MergeStake),
    ResetStake(reset_stake::ResetStake),
    WithdrawStake(withdraw_stake::WithdrawStake),
    EmergencyPause(emergency_pause::EmergencyPause),
    EmergencyResume(emergency_resume::EmergencyResume),
    CloseSettlementV2(close_settlement_v2::CloseSettlementV2),
    ClaimSettlementV2(claim_settlement_v2::ClaimSettlementV2),
    InitBondEvent(init_bond_event::InitBondEvent),
    ConfigureBondEvent(configure_bond_event::ConfigureBondEvent),
    ConfigureBondWithMintEvent(configure_bond_with_mint_event::ConfigureBondWithMintEvent),
    FundBondEvent(fund_bond_event::FundBondEvent),
    MintBondEvent(mint_bond_event::MintBondEvent),
    InitConfigEvent(init_config_event::InitConfigEvent),
    ConfigureConfigEvent(configure_config_event::ConfigureConfigEvent),
    EmergencyPauseEvent(emergency_pause_event::EmergencyPauseEvent),
    EmergencyResumeEvent(emergency_resume_event::EmergencyResumeEvent),
    ClaimSettlementV2Event(claim_settlement_v2_event::ClaimSettlementV2Event),
    InitSettlementEvent(init_settlement_event::InitSettlementEvent),
    CloseSettlementEvent(close_settlement_event::CloseSettlementEvent),
    CancelSettlementEvent(cancel_settlement_event::CancelSettlementEvent),
    FundSettlementEvent(fund_settlement_event::FundSettlementEvent),
    MergeStakeEvent(merge_stake_event::MergeStakeEvent),
    ResetStakeEvent(reset_stake_event::ResetStakeEvent),
    WithdrawStakeEvent(withdraw_stake_event::WithdrawStakeEvent),
    InitWithdrawRequestEvent(init_withdraw_request_event::InitWithdrawRequestEvent),
    CancelWithdrawRequestEvent(cancel_withdraw_request_event::CancelWithdrawRequestEvent),
    ClaimWithdrawRequestEvent(claim_withdraw_request_event::ClaimWithdrawRequestEvent),
    ClaimSettlementEvent(claim_settlement_event::ClaimSettlementEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for ValidatorBondsDecoder {
    type InstructionType = ValidatorBondsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            ValidatorBondsInstruction::InitConfig => init_config::InitConfig,
            ValidatorBondsInstruction::ConfigureConfig => configure_config::ConfigureConfig,
            ValidatorBondsInstruction::InitBond => init_bond::InitBond,
            ValidatorBondsInstruction::ConfigureBond => configure_bond::ConfigureBond,
            ValidatorBondsInstruction::ConfigureBondWithMint => configure_bond_with_mint::ConfigureBondWithMint,
            ValidatorBondsInstruction::MintBond => mint_bond::MintBond,
            ValidatorBondsInstruction::FundBond => fund_bond::FundBond,
            ValidatorBondsInstruction::InitWithdrawRequest => init_withdraw_request::InitWithdrawRequest,
            ValidatorBondsInstruction::CancelWithdrawRequest => cancel_withdraw_request::CancelWithdrawRequest,
            ValidatorBondsInstruction::ClaimWithdrawRequest => claim_withdraw_request::ClaimWithdrawRequest,
            ValidatorBondsInstruction::InitSettlement => init_settlement::InitSettlement,
            ValidatorBondsInstruction::UpsizeSettlementClaims => upsize_settlement_claims::UpsizeSettlementClaims,
            ValidatorBondsInstruction::CancelSettlement => cancel_settlement::CancelSettlement,
            ValidatorBondsInstruction::FundSettlement => fund_settlement::FundSettlement,
            ValidatorBondsInstruction::MergeStake => merge_stake::MergeStake,
            ValidatorBondsInstruction::ResetStake => reset_stake::ResetStake,
            ValidatorBondsInstruction::WithdrawStake => withdraw_stake::WithdrawStake,
            ValidatorBondsInstruction::EmergencyPause => emergency_pause::EmergencyPause,
            ValidatorBondsInstruction::EmergencyResume => emergency_resume::EmergencyResume,
            ValidatorBondsInstruction::CloseSettlementV2 => close_settlement_v2::CloseSettlementV2,
            ValidatorBondsInstruction::ClaimSettlementV2 => claim_settlement_v2::ClaimSettlementV2,
            ValidatorBondsInstruction::InitBondEvent => init_bond_event::InitBondEvent,
            ValidatorBondsInstruction::ConfigureBondEvent => configure_bond_event::ConfigureBondEvent,
            ValidatorBondsInstruction::ConfigureBondWithMintEvent => configure_bond_with_mint_event::ConfigureBondWithMintEvent,
            ValidatorBondsInstruction::FundBondEvent => fund_bond_event::FundBondEvent,
            ValidatorBondsInstruction::MintBondEvent => mint_bond_event::MintBondEvent,
            ValidatorBondsInstruction::InitConfigEvent => init_config_event::InitConfigEvent,
            ValidatorBondsInstruction::ConfigureConfigEvent => configure_config_event::ConfigureConfigEvent,
            ValidatorBondsInstruction::EmergencyPauseEvent => emergency_pause_event::EmergencyPauseEvent,
            ValidatorBondsInstruction::EmergencyResumeEvent => emergency_resume_event::EmergencyResumeEvent,
            ValidatorBondsInstruction::ClaimSettlementV2Event => claim_settlement_v2_event::ClaimSettlementV2Event,
            ValidatorBondsInstruction::InitSettlementEvent => init_settlement_event::InitSettlementEvent,
            ValidatorBondsInstruction::CloseSettlementEvent => close_settlement_event::CloseSettlementEvent,
            ValidatorBondsInstruction::CancelSettlementEvent => cancel_settlement_event::CancelSettlementEvent,
            ValidatorBondsInstruction::FundSettlementEvent => fund_settlement_event::FundSettlementEvent,
            ValidatorBondsInstruction::MergeStakeEvent => merge_stake_event::MergeStakeEvent,
            ValidatorBondsInstruction::ResetStakeEvent => reset_stake_event::ResetStakeEvent,
            ValidatorBondsInstruction::WithdrawStakeEvent => withdraw_stake_event::WithdrawStakeEvent,
            ValidatorBondsInstruction::InitWithdrawRequestEvent => init_withdraw_request_event::InitWithdrawRequestEvent,
            ValidatorBondsInstruction::CancelWithdrawRequestEvent => cancel_withdraw_request_event::CancelWithdrawRequestEvent,
            ValidatorBondsInstruction::ClaimWithdrawRequestEvent => claim_withdraw_request_event::ClaimWithdrawRequestEvent,
            ValidatorBondsInstruction::ClaimSettlementEvent => claim_settlement_event::ClaimSettlementEvent,
        )
    }
}
