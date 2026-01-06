use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xcf3aba704014bcd4")]
pub struct RebateVault {
    pub key: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub is_initialized: bool,
    pub allow_payout: bool,
    pub bump: u8,
    pub token_account_bump: u8,
    pub available_usd: u64,
    pub available_amount: u64,
    pub padding: [u64; 4],
}
