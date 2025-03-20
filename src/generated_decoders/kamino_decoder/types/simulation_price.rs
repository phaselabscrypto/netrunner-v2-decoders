

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum SimulationPrice {
    PoolPrice,
    SqrtPrice
                (
                    u128,
                )
    ,
    TickIndex
                (
                    i32,
                )
    ,
}


