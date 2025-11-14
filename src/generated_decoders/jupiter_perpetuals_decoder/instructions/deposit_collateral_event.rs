use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da90e66949b8912eb")]
pub struct DepositCollateralEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position_key: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_custody: solana_sdk::pubkey::Pubkey,
    pub deposit_amount: u64,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub time: i64,
}
