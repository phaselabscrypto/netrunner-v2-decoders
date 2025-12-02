use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum VoltagePointsType {
    OpenPosition,
    SwapAndOpen,
    IncreaseSize,
    DecreaseSize,
    CloseAndSwap,
    ClosePosition,
    ExecuteLimitOrder,
    ExecuteLimitWithSwap,
    ExecuteTriggerOrder,
    ExecuteTriggerWithSwap,
    CollectStakeReward,
    DecreaseAndRemoveCollateral,
}
