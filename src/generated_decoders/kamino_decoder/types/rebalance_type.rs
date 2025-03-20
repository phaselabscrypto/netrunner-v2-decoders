

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum RebalanceType {
    Manual,
    PricePercentage,
    PricePercentageWithReset,
    Drift,
    TakeProfit,
    PeriodicRebalance,
    Expander,
    Autodrift,
}


