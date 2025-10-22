use super::ByrealClmmDecoder;
pub mod claim_offchain_reward;
pub mod close_position;
pub mod collect_fund_fee;
pub mod collect_personal_fee_event;
pub mod collect_protocol_fee;
pub mod collect_protocol_fee_event;
pub mod collect_remaining_rewards;
pub mod config_change_event;
pub mod create_amm_config;
pub mod create_operation_account;
pub mod create_personal_position_event;
pub mod create_pool;
pub mod create_support_mint_associated;
pub mod decrease_liquidity;
pub mod decrease_liquidity_event;
pub mod decrease_liquidity_v2;
pub mod deposit_offchain_reward;
pub mod increase_liquidity;
pub mod increase_liquidity_event;
pub mod increase_liquidity_v2;
pub mod init_amm_admin_group;
pub mod initialize_reward;
pub mod liquidity_calculate_event;
pub mod liquidity_change_event;
pub mod modify_amm_admin_group_event;
pub mod open_position;
pub mod open_position_v2;
pub mod open_position_with_token22_nft;
pub mod pool_created_event;
pub mod set_reward_params;
pub mod swap;
pub mod swap_event;
pub mod swap_v2;
pub mod transfer_reward_owner;
pub mod update_amm_admin_group;
pub mod update_amm_config;
pub mod update_operation_account;
pub mod update_pool_status;
pub mod update_reward_infos;
pub mod update_reward_infos_event;
pub mod withdraw_offchain_reward;

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
pub enum ByrealClmmInstruction {
    ClaimOffchainReward(claim_offchain_reward::ClaimOffchainReward),
    ClosePosition(close_position::ClosePosition),
    CollectFundFee(collect_fund_fee::CollectFundFee),
    CollectProtocolFee(collect_protocol_fee::CollectProtocolFee),
    CollectRemainingRewards(collect_remaining_rewards::CollectRemainingRewards),
    CreateAmmConfig(create_amm_config::CreateAmmConfig),
    CreateOperationAccount(create_operation_account::CreateOperationAccount),
    CreatePool(create_pool::CreatePool),
    CreateSupportMintAssociated(create_support_mint_associated::CreateSupportMintAssociated),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    DepositOffchainReward(deposit_offchain_reward::DepositOffchainReward),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    InitAmmAdminGroup(init_amm_admin_group::InitAmmAdminGroup),
    InitializeReward(initialize_reward::InitializeReward),
    OpenPosition(open_position::OpenPosition),
    OpenPositionV2(open_position_v2::OpenPositionV2),
    OpenPositionWithToken22Nft(open_position_with_token22_nft::OpenPositionWithToken22Nft),
    SetRewardParams(set_reward_params::SetRewardParams),
    Swap(swap::Swap),
    SwapV2(swap_v2::SwapV2),
    TransferRewardOwner(transfer_reward_owner::TransferRewardOwner),
    UpdateAmmAdminGroup(update_amm_admin_group::UpdateAmmAdminGroup),
    UpdateAmmConfig(update_amm_config::UpdateAmmConfig),
    UpdateOperationAccount(update_operation_account::UpdateOperationAccount),
    UpdatePoolStatus(update_pool_status::UpdatePoolStatus),
    UpdateRewardInfos(update_reward_infos::UpdateRewardInfos),
    WithdrawOffchainReward(withdraw_offchain_reward::WithdrawOffchainReward),
    CollectPersonalFeeEvent(collect_personal_fee_event::CollectPersonalFeeEvent),
    CollectProtocolFeeEvent(collect_protocol_fee_event::CollectProtocolFeeEvent),
    ConfigChangeEvent(config_change_event::ConfigChangeEvent),
    CreatePersonalPositionEvent(create_personal_position_event::CreatePersonalPositionEvent),
    DecreaseLiquidityEvent(decrease_liquidity_event::DecreaseLiquidityEvent),
    IncreaseLiquidityEvent(increase_liquidity_event::IncreaseLiquidityEvent),
    LiquidityCalculateEvent(liquidity_calculate_event::LiquidityCalculateEvent),
    LiquidityChangeEvent(liquidity_change_event::LiquidityChangeEvent),
    ModifyAmmAdminGroupEvent(modify_amm_admin_group_event::ModifyAmmAdminGroupEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    SwapEvent(swap_event::SwapEvent),
    UpdateRewardInfosEvent(update_reward_infos_event::UpdateRewardInfosEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for ByrealClmmDecoder {
    type InstructionType = ByrealClmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            ByrealClmmInstruction::ClaimOffchainReward => claim_offchain_reward::ClaimOffchainReward,
            ByrealClmmInstruction::ClosePosition => close_position::ClosePosition,
            ByrealClmmInstruction::CollectFundFee => collect_fund_fee::CollectFundFee,
            ByrealClmmInstruction::CollectProtocolFee => collect_protocol_fee::CollectProtocolFee,
            ByrealClmmInstruction::CollectRemainingRewards => collect_remaining_rewards::CollectRemainingRewards,
            ByrealClmmInstruction::CreateAmmConfig => create_amm_config::CreateAmmConfig,
            ByrealClmmInstruction::CreateOperationAccount => create_operation_account::CreateOperationAccount,
            ByrealClmmInstruction::CreatePool => create_pool::CreatePool,
            ByrealClmmInstruction::CreateSupportMintAssociated => create_support_mint_associated::CreateSupportMintAssociated,
            ByrealClmmInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            ByrealClmmInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            ByrealClmmInstruction::DepositOffchainReward => deposit_offchain_reward::DepositOffchainReward,
            ByrealClmmInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            ByrealClmmInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            ByrealClmmInstruction::InitAmmAdminGroup => init_amm_admin_group::InitAmmAdminGroup,
            ByrealClmmInstruction::InitializeReward => initialize_reward::InitializeReward,
            ByrealClmmInstruction::OpenPosition => open_position::OpenPosition,
            ByrealClmmInstruction::OpenPositionV2 => open_position_v2::OpenPositionV2,
            ByrealClmmInstruction::OpenPositionWithToken22Nft => open_position_with_token22_nft::OpenPositionWithToken22Nft,
            ByrealClmmInstruction::SetRewardParams => set_reward_params::SetRewardParams,
            ByrealClmmInstruction::Swap => swap::Swap,
            ByrealClmmInstruction::SwapV2 => swap_v2::SwapV2,
            ByrealClmmInstruction::TransferRewardOwner => transfer_reward_owner::TransferRewardOwner,
            ByrealClmmInstruction::UpdateAmmAdminGroup => update_amm_admin_group::UpdateAmmAdminGroup,
            ByrealClmmInstruction::UpdateAmmConfig => update_amm_config::UpdateAmmConfig,
            ByrealClmmInstruction::UpdateOperationAccount => update_operation_account::UpdateOperationAccount,
            ByrealClmmInstruction::UpdatePoolStatus => update_pool_status::UpdatePoolStatus,
            ByrealClmmInstruction::UpdateRewardInfos => update_reward_infos::UpdateRewardInfos,
            ByrealClmmInstruction::WithdrawOffchainReward => withdraw_offchain_reward::WithdrawOffchainReward,
            ByrealClmmInstruction::CollectPersonalFeeEvent => collect_personal_fee_event::CollectPersonalFeeEvent,
            ByrealClmmInstruction::CollectProtocolFeeEvent => collect_protocol_fee_event::CollectProtocolFeeEvent,
            ByrealClmmInstruction::ConfigChangeEvent => config_change_event::ConfigChangeEvent,
            ByrealClmmInstruction::CreatePersonalPositionEvent => create_personal_position_event::CreatePersonalPositionEvent,
            ByrealClmmInstruction::DecreaseLiquidityEvent => decrease_liquidity_event::DecreaseLiquidityEvent,
            ByrealClmmInstruction::IncreaseLiquidityEvent => increase_liquidity_event::IncreaseLiquidityEvent,
            ByrealClmmInstruction::LiquidityCalculateEvent => liquidity_calculate_event::LiquidityCalculateEvent,
            ByrealClmmInstruction::LiquidityChangeEvent => liquidity_change_event::LiquidityChangeEvent,
            ByrealClmmInstruction::ModifyAmmAdminGroupEvent => modify_amm_admin_group_event::ModifyAmmAdminGroupEvent,
            ByrealClmmInstruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            ByrealClmmInstruction::SwapEvent => swap_event::SwapEvent,
            ByrealClmmInstruction::UpdateRewardInfosEvent => update_reward_infos_event::UpdateRewardInfosEvent,
        )
    }
}
