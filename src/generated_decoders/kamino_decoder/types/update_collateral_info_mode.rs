use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum UpdateCollateralInfoMode {
    CollateralId,
    LowerHeuristic,
    UpperHeuristic,
    ExpHeuristic,
    TwapDivergence,
    UpdateScopeTwap,
    UpdateScopeChain,
    UpdateName,
    UpdatePriceMaxAge,
    UpdateTwapMaxAge,
    UpdateDisabled,
    UpdateStakingRateChain,
    UpdateMaxIgnorableAmountAsReward,
}
