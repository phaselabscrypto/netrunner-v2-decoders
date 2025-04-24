use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SimulationPrice {
    PoolPrice,
    SqrtPrice(u128),
    TickIndex(i32),
}
