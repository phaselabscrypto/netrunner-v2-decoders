use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct BackupOracle {
    pub price: i64,
    pub expo: i32,
    pub conf: i64,
    pub ema_price: i64,
    pub publish_time: i64,
}
