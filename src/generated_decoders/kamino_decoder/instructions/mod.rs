use super::YvaultsDecoder;
pub mod add_kamino_rewards;
pub mod change_pool;
pub mod check_expected_vaults_balances;
pub mod close_program_account;
pub mod close_strategy;
pub mod collect_fees_and_rewards;
pub mod deposit;
pub mod deposit_and_invest;
pub mod emergency_swap;
pub mod executive_withdraw;
pub mod flash_swap_uneven_vaults_end;
pub mod flash_swap_uneven_vaults_start;
pub mod initialize_collateral_info;
pub mod initialize_global_config;
pub mod initialize_kamino_reward;
pub mod initialize_shares_metadata;
pub mod initialize_strategy;
pub mod insert_collateral_info;
pub mod invest;
pub mod open_liquidity_position;
pub mod orca_swap;
pub mod permisionless_withdraw_from_treasury;
pub mod sign_terms;
pub mod single_token_deposit_and_invest_with_min;
pub mod single_token_deposit_with_min;
pub mod swap_rewards;
pub mod update_collateral_info;
pub mod update_global_config;
pub mod update_reward_mapping;
pub mod update_shares_metadata;
pub mod update_strategy_admin;
pub mod update_strategy_config;
pub mod update_treasury_fee_vault;
pub mod withdraw;
pub mod withdraw_from_topup;
pub mod withdraw_from_treasury;

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
pub enum YvaultsInstruction {
    InitializeStrategy(initialize_strategy::InitializeStrategy),
    InitializeKaminoReward(initialize_kamino_reward::InitializeKaminoReward),
    AddKaminoRewards(add_kamino_rewards::AddKaminoRewards),
    InitializeGlobalConfig(initialize_global_config::InitializeGlobalConfig),
    InitializeCollateralInfo(initialize_collateral_info::InitializeCollateralInfo),
    UpdateCollateralInfo(update_collateral_info::UpdateCollateralInfo),
    InsertCollateralInfo(insert_collateral_info::InsertCollateralInfo),
    InitializeSharesMetadata(initialize_shares_metadata::InitializeSharesMetadata),
    UpdateSharesMetadata(update_shares_metadata::UpdateSharesMetadata),
    UpdateGlobalConfig(update_global_config::UpdateGlobalConfig),
    UpdateTreasuryFeeVault(update_treasury_fee_vault::UpdateTreasuryFeeVault),
    UpdateStrategyConfig(update_strategy_config::UpdateStrategyConfig),
    UpdateRewardMapping(update_reward_mapping::UpdateRewardMapping),
    OpenLiquidityPosition(open_liquidity_position::OpenLiquidityPosition),
    CloseStrategy(close_strategy::CloseStrategy),
    Deposit(deposit::Deposit),
    Invest(invest::Invest),
    DepositAndInvest(deposit_and_invest::DepositAndInvest),
    Withdraw(withdraw::Withdraw),
    ExecutiveWithdraw(executive_withdraw::ExecutiveWithdraw),
    CollectFeesAndRewards(collect_fees_and_rewards::CollectFeesAndRewards),
    SwapRewards(swap_rewards::SwapRewards),
    CheckExpectedVaultsBalances(check_expected_vaults_balances::CheckExpectedVaultsBalances),
    SingleTokenDepositAndInvestWithMin(
        single_token_deposit_and_invest_with_min::SingleTokenDepositAndInvestWithMin,
    ),
    SingleTokenDepositWithMin(single_token_deposit_with_min::SingleTokenDepositWithMin),
    FlashSwapUnevenVaultsStart(flash_swap_uneven_vaults_start::FlashSwapUnevenVaultsStart),
    FlashSwapUnevenVaultsEnd(flash_swap_uneven_vaults_end::FlashSwapUnevenVaultsEnd),
    EmergencySwap(emergency_swap::EmergencySwap),
    WithdrawFromTreasury(withdraw_from_treasury::WithdrawFromTreasury),
    PermisionlessWithdrawFromTreasury(
        permisionless_withdraw_from_treasury::PermisionlessWithdrawFromTreasury,
    ),
    WithdrawFromTopup(withdraw_from_topup::WithdrawFromTopup),
    ChangePool(change_pool::ChangePool),
    CloseProgramAccount(close_program_account::CloseProgramAccount),
    OrcaSwap(orca_swap::OrcaSwap),
    SignTerms(sign_terms::SignTerms),
    UpdateStrategyAdmin(update_strategy_admin::UpdateStrategyAdmin),
}

impl carbon_core::instruction::InstructionDecoder<'_> for YvaultsDecoder {
    type InstructionType = YvaultsInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            YvaultsInstruction::InitializeStrategy => initialize_strategy::InitializeStrategy,
            YvaultsInstruction::InitializeKaminoReward => initialize_kamino_reward::InitializeKaminoReward,
            YvaultsInstruction::AddKaminoRewards => add_kamino_rewards::AddKaminoRewards,
            YvaultsInstruction::InitializeGlobalConfig => initialize_global_config::InitializeGlobalConfig,
            YvaultsInstruction::InitializeCollateralInfo => initialize_collateral_info::InitializeCollateralInfo,
            YvaultsInstruction::UpdateCollateralInfo => update_collateral_info::UpdateCollateralInfo,
            YvaultsInstruction::InsertCollateralInfo => insert_collateral_info::InsertCollateralInfo,
            YvaultsInstruction::InitializeSharesMetadata => initialize_shares_metadata::InitializeSharesMetadata,
            YvaultsInstruction::UpdateSharesMetadata => update_shares_metadata::UpdateSharesMetadata,
            YvaultsInstruction::UpdateGlobalConfig => update_global_config::UpdateGlobalConfig,
            YvaultsInstruction::UpdateTreasuryFeeVault => update_treasury_fee_vault::UpdateTreasuryFeeVault,
            YvaultsInstruction::UpdateStrategyConfig => update_strategy_config::UpdateStrategyConfig,
            YvaultsInstruction::UpdateRewardMapping => update_reward_mapping::UpdateRewardMapping,
            YvaultsInstruction::OpenLiquidityPosition => open_liquidity_position::OpenLiquidityPosition,
            YvaultsInstruction::CloseStrategy => close_strategy::CloseStrategy,
            YvaultsInstruction::Deposit => deposit::Deposit,
            YvaultsInstruction::Invest => invest::Invest,
            YvaultsInstruction::DepositAndInvest => deposit_and_invest::DepositAndInvest,
            YvaultsInstruction::Withdraw => withdraw::Withdraw,
            YvaultsInstruction::ExecutiveWithdraw => executive_withdraw::ExecutiveWithdraw,
            YvaultsInstruction::CollectFeesAndRewards => collect_fees_and_rewards::CollectFeesAndRewards,
            YvaultsInstruction::SwapRewards => swap_rewards::SwapRewards,
            YvaultsInstruction::CheckExpectedVaultsBalances => check_expected_vaults_balances::CheckExpectedVaultsBalances,
            YvaultsInstruction::SingleTokenDepositAndInvestWithMin => single_token_deposit_and_invest_with_min::SingleTokenDepositAndInvestWithMin,
            YvaultsInstruction::SingleTokenDepositWithMin => single_token_deposit_with_min::SingleTokenDepositWithMin,
            YvaultsInstruction::FlashSwapUnevenVaultsStart => flash_swap_uneven_vaults_start::FlashSwapUnevenVaultsStart,
            YvaultsInstruction::FlashSwapUnevenVaultsEnd => flash_swap_uneven_vaults_end::FlashSwapUnevenVaultsEnd,
            YvaultsInstruction::EmergencySwap => emergency_swap::EmergencySwap,
            YvaultsInstruction::WithdrawFromTreasury => withdraw_from_treasury::WithdrawFromTreasury,
            YvaultsInstruction::PermisionlessWithdrawFromTreasury => permisionless_withdraw_from_treasury::PermisionlessWithdrawFromTreasury,
            YvaultsInstruction::WithdrawFromTopup => withdraw_from_topup::WithdrawFromTopup,
            YvaultsInstruction::ChangePool => change_pool::ChangePool,
            YvaultsInstruction::CloseProgramAccount => close_program_account::CloseProgramAccount,
            YvaultsInstruction::OrcaSwap => orca_swap::OrcaSwap,
            YvaultsInstruction::SignTerms => sign_terms::SignTerms,
            YvaultsInstruction::UpdateStrategyAdmin => update_strategy_admin::UpdateStrategyAdmin,
        )
    }
}
