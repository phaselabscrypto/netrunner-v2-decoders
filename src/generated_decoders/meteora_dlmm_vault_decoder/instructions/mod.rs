



use super::AlphaVaultDecoder;
pub mod transfer_vault_authority;
pub mod initialize_prorata_vault;
pub mod initialize_vault_with_prorata_config;
pub mod update_prorata_vault_parameters;
pub mod create_prorata_config;
pub mod close_prorata_config;
pub mod initialize_fcfs_vault;
pub mod initialize_vault_with_fcfs_config;
pub mod update_fcfs_vault_parameters;
pub mod create_fcfs_config;
pub mod close_fcfs_config;
pub mod create_merkle_root_config;
pub mod create_new_escrow;
pub mod create_permissioned_escrow;
pub mod create_permissioned_escrow_with_authority;
pub mod close_escrow;
pub mod deposit;
pub mod withdraw;
pub mod withdraw_remaining_quote;
pub mod claim_token;
pub mod fill_dlmm;
pub mod fill_dynamic_amm;
pub mod prorata_vault_created_event;
pub mod fcfs_vault_created_event;
pub mod escrow_created_event;
pub mod merkle_root_config_created_event;
pub mod prorata_vault_parameters_updated_event;
pub mod fcfs_vault_parameters_updated_event;
pub mod escrow_remaining_withdraw_event;
pub mod escrow_withdraw_event;
pub mod swap_fill_event;
pub mod escrow_deposit_event;
pub mod escrow_closed_event;
pub mod escrow_claim_token_event;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum AlphaVaultInstruction {
    TransferVaultAuthority(transfer_vault_authority::TransferVaultAuthority),
    InitializeProrataVault(initialize_prorata_vault::InitializeProrataVault),
    InitializeVaultWithProrataConfig(initialize_vault_with_prorata_config::InitializeVaultWithProrataConfig),
    UpdateProrataVaultParameters(update_prorata_vault_parameters::UpdateProrataVaultParameters),
    CreateProrataConfig(create_prorata_config::CreateProrataConfig),
    CloseProrataConfig(close_prorata_config::CloseProrataConfig),
    InitializeFcfsVault(initialize_fcfs_vault::InitializeFcfsVault),
    InitializeVaultWithFcfsConfig(initialize_vault_with_fcfs_config::InitializeVaultWithFcfsConfig),
    UpdateFcfsVaultParameters(update_fcfs_vault_parameters::UpdateFcfsVaultParameters),
    CreateFcfsConfig(create_fcfs_config::CreateFcfsConfig),
    CloseFcfsConfig(close_fcfs_config::CloseFcfsConfig),
    CreateMerkleRootConfig(create_merkle_root_config::CreateMerkleRootConfig),
    CreateNewEscrow(create_new_escrow::CreateNewEscrow),
    CreatePermissionedEscrow(create_permissioned_escrow::CreatePermissionedEscrow),
    CreatePermissionedEscrowWithAuthority(create_permissioned_escrow_with_authority::CreatePermissionedEscrowWithAuthority),
    CloseEscrow(close_escrow::CloseEscrow),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    WithdrawRemainingQuote(withdraw_remaining_quote::WithdrawRemainingQuote),
    ClaimToken(claim_token::ClaimToken),
    FillDlmm(fill_dlmm::FillDlmm),
    FillDynamicAmm(fill_dynamic_amm::FillDynamicAmm),
    ProrataVaultCreatedEvent(prorata_vault_created_event::ProrataVaultCreatedEvent),
    FcfsVaultCreatedEvent(fcfs_vault_created_event::FcfsVaultCreatedEvent),
    EscrowCreatedEvent(escrow_created_event::EscrowCreatedEvent),
    MerkleRootConfigCreatedEvent(merkle_root_config_created_event::MerkleRootConfigCreatedEvent),
    ProrataVaultParametersUpdatedEvent(prorata_vault_parameters_updated_event::ProrataVaultParametersUpdatedEvent),
    FcfsVaultParametersUpdatedEvent(fcfs_vault_parameters_updated_event::FcfsVaultParametersUpdatedEvent),
    EscrowRemainingWithdrawEvent(escrow_remaining_withdraw_event::EscrowRemainingWithdrawEvent),
    EscrowWithdrawEvent(escrow_withdraw_event::EscrowWithdrawEvent),
    SwapFillEvent(swap_fill_event::SwapFillEvent),
    EscrowDepositEvent(escrow_deposit_event::EscrowDepositEvent),
    EscrowClosedEvent(escrow_closed_event::EscrowClosedEvent),
    EscrowClaimTokenEvent(escrow_claim_token_event::EscrowClaimTokenEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for AlphaVaultDecoder {
    type InstructionType = AlphaVaultInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            AlphaVaultInstruction::TransferVaultAuthority => transfer_vault_authority::TransferVaultAuthority,
            AlphaVaultInstruction::InitializeProrataVault => initialize_prorata_vault::InitializeProrataVault,
            AlphaVaultInstruction::InitializeVaultWithProrataConfig => initialize_vault_with_prorata_config::InitializeVaultWithProrataConfig,
            AlphaVaultInstruction::UpdateProrataVaultParameters => update_prorata_vault_parameters::UpdateProrataVaultParameters,
            AlphaVaultInstruction::CreateProrataConfig => create_prorata_config::CreateProrataConfig,
            AlphaVaultInstruction::CloseProrataConfig => close_prorata_config::CloseProrataConfig,
            AlphaVaultInstruction::InitializeFcfsVault => initialize_fcfs_vault::InitializeFcfsVault,
            AlphaVaultInstruction::InitializeVaultWithFcfsConfig => initialize_vault_with_fcfs_config::InitializeVaultWithFcfsConfig,
            AlphaVaultInstruction::UpdateFcfsVaultParameters => update_fcfs_vault_parameters::UpdateFcfsVaultParameters,
            AlphaVaultInstruction::CreateFcfsConfig => create_fcfs_config::CreateFcfsConfig,
            AlphaVaultInstruction::CloseFcfsConfig => close_fcfs_config::CloseFcfsConfig,
            AlphaVaultInstruction::CreateMerkleRootConfig => create_merkle_root_config::CreateMerkleRootConfig,
            AlphaVaultInstruction::CreateNewEscrow => create_new_escrow::CreateNewEscrow,
            AlphaVaultInstruction::CreatePermissionedEscrow => create_permissioned_escrow::CreatePermissionedEscrow,
            AlphaVaultInstruction::CreatePermissionedEscrowWithAuthority => create_permissioned_escrow_with_authority::CreatePermissionedEscrowWithAuthority,
            AlphaVaultInstruction::CloseEscrow => close_escrow::CloseEscrow,
            AlphaVaultInstruction::Deposit => deposit::Deposit,
            AlphaVaultInstruction::Withdraw => withdraw::Withdraw,
            AlphaVaultInstruction::WithdrawRemainingQuote => withdraw_remaining_quote::WithdrawRemainingQuote,
            AlphaVaultInstruction::ClaimToken => claim_token::ClaimToken,
            AlphaVaultInstruction::FillDlmm => fill_dlmm::FillDlmm,
            AlphaVaultInstruction::FillDynamicAmm => fill_dynamic_amm::FillDynamicAmm,
            AlphaVaultInstruction::ProrataVaultCreatedEvent => prorata_vault_created_event::ProrataVaultCreatedEvent,
            AlphaVaultInstruction::FcfsVaultCreatedEvent => fcfs_vault_created_event::FcfsVaultCreatedEvent,
            AlphaVaultInstruction::EscrowCreatedEvent => escrow_created_event::EscrowCreatedEvent,
            AlphaVaultInstruction::MerkleRootConfigCreatedEvent => merkle_root_config_created_event::MerkleRootConfigCreatedEvent,
            AlphaVaultInstruction::ProrataVaultParametersUpdatedEvent => prorata_vault_parameters_updated_event::ProrataVaultParametersUpdatedEvent,
            AlphaVaultInstruction::FcfsVaultParametersUpdatedEvent => fcfs_vault_parameters_updated_event::FcfsVaultParametersUpdatedEvent,
            AlphaVaultInstruction::EscrowRemainingWithdrawEvent => escrow_remaining_withdraw_event::EscrowRemainingWithdrawEvent,
            AlphaVaultInstruction::EscrowWithdrawEvent => escrow_withdraw_event::EscrowWithdrawEvent,
            AlphaVaultInstruction::SwapFillEvent => swap_fill_event::SwapFillEvent,
            AlphaVaultInstruction::EscrowDepositEvent => escrow_deposit_event::EscrowDepositEvent,
            AlphaVaultInstruction::EscrowClosedEvent => escrow_closed_event::EscrowClosedEvent,
            AlphaVaultInstruction::EscrowClaimTokenEvent => escrow_claim_token_event::EscrowClaimTokenEvent,
        )
    }
}