



use super::VaultDecoder;
pub mod initialize;
pub mod enable_vault;
pub mod set_operator;
pub mod initialize_strategy;
pub mod remove_strategy;
pub mod remove_strategy2;
pub mod collect_dust;
pub mod add_strategy;
pub mod deposit_strategy;
pub mod withdraw_strategy;
pub mod withdraw2;
pub mod deposit;
pub mod withdraw;
pub mod withdraw_directly_from_strategy;
pub mod add_liquidity_event;
pub mod remove_liquidity_event;
pub mod strategy_deposit_event;
pub mod strategy_withdraw_event;
pub mod claim_reward_event;
pub mod performance_fee_event;
pub mod report_loss_event;
pub mod total_amount_event;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum VaultInstruction {
    Initialize(initialize::Initialize),
    EnableVault(enable_vault::EnableVault),
    SetOperator(set_operator::SetOperator),
    InitializeStrategy(initialize_strategy::InitializeStrategy),
    RemoveStrategy(remove_strategy::RemoveStrategy),
    RemoveStrategy2(remove_strategy2::RemoveStrategy2),
    CollectDust(collect_dust::CollectDust),
    AddStrategy(add_strategy::AddStrategy),
    DepositStrategy(deposit_strategy::DepositStrategy),
    WithdrawStrategy(withdraw_strategy::WithdrawStrategy),
    Withdraw2(withdraw2::Withdraw2),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    WithdrawDirectlyFromStrategy(withdraw_directly_from_strategy::WithdrawDirectlyFromStrategy),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    StrategyDepositEvent(strategy_deposit_event::StrategyDepositEvent),
    StrategyWithdrawEvent(strategy_withdraw_event::StrategyWithdrawEvent),
    ClaimRewardEvent(claim_reward_event::ClaimRewardEvent),
    PerformanceFeeEvent(performance_fee_event::PerformanceFeeEvent),
    ReportLossEvent(report_loss_event::ReportLossEvent),
    TotalAmountEvent(total_amount_event::TotalAmountEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for VaultDecoder {
    type InstructionType = VaultInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            VaultInstruction::Initialize => initialize::Initialize,
            VaultInstruction::EnableVault => enable_vault::EnableVault,
            VaultInstruction::SetOperator => set_operator::SetOperator,
            VaultInstruction::InitializeStrategy => initialize_strategy::InitializeStrategy,
            VaultInstruction::RemoveStrategy => remove_strategy::RemoveStrategy,
            VaultInstruction::RemoveStrategy2 => remove_strategy2::RemoveStrategy2,
            VaultInstruction::CollectDust => collect_dust::CollectDust,
            VaultInstruction::AddStrategy => add_strategy::AddStrategy,
            VaultInstruction::DepositStrategy => deposit_strategy::DepositStrategy,
            VaultInstruction::WithdrawStrategy => withdraw_strategy::WithdrawStrategy,
            VaultInstruction::Withdraw2 => withdraw2::Withdraw2,
            VaultInstruction::Deposit => deposit::Deposit,
            VaultInstruction::Withdraw => withdraw::Withdraw,
            VaultInstruction::WithdrawDirectlyFromStrategy => withdraw_directly_from_strategy::WithdrawDirectlyFromStrategy,
            VaultInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            VaultInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            VaultInstruction::StrategyDepositEvent => strategy_deposit_event::StrategyDepositEvent,
            VaultInstruction::StrategyWithdrawEvent => strategy_withdraw_event::StrategyWithdrawEvent,
            VaultInstruction::ClaimRewardEvent => claim_reward_event::ClaimRewardEvent,
            VaultInstruction::PerformanceFeeEvent => performance_fee_event::PerformanceFeeEvent,
            VaultInstruction::ReportLossEvent => report_loss_event::ReportLossEvent,
            VaultInstruction::TotalAmountEvent => total_amount_event::TotalAmountEvent,
        )
    }
}