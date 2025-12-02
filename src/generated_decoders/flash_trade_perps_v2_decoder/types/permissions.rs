use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Permissions {
    pub allow_swap: bool,
    pub allow_add_liquidity: bool,
    pub allow_remove_liquidity: bool,
    pub allow_open_position: bool,
    pub allow_close_position: bool,
    pub allow_collateral_withdrawal: bool,
    pub allow_size_change: bool,
    pub allow_liquidation: bool,
    pub allow_lp_staking: bool,
    pub allow_fee_distribution: bool,
    pub allow_ungated_trading: bool,
    pub allow_fee_discounts: bool,
    pub allow_referral_rebates: bool,
}
