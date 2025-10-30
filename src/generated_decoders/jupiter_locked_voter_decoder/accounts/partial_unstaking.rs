use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xac923ad528fa6b3f")]
pub struct PartialUnstaking {
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub expiration: i64,
    pub buffers: [u128; 6],
    pub memo: String,
}
