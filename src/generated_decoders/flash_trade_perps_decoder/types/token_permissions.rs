use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenPermissions {
    pub allow_deposits: bool,
    pub allow_withdrawal: bool,
    pub allow_reward_withdrawal: bool,
}
