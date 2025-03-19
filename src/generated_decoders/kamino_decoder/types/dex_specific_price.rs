use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum DexSpecificPrice {
    SqrtPrice(u128),
    Q64_64(u128),
}
