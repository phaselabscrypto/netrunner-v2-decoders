use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xc8a7c5ee208b1a45")]
pub struct ProtocolVault {
    pub key: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub is_initialized: bool,
    pub bump: u8,
    pub token_account_bump: u8,
    pub fee_share_bps: u64,
    pub fee_amount: u64,
    pub padding: [u64; 4],
}
