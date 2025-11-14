use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x1eeb88e06a6b3140")]
pub struct Referral {
    pub is_initialized: bool,
    pub bump: u8,
    pub referer_token_stake_account: solana_sdk::pubkey::Pubkey,
    pub referer_booster_account: solana_sdk::pubkey::Pubkey,
    pub padding: [u64; 4],
}
