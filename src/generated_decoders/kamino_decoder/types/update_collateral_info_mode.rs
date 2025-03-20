

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
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


