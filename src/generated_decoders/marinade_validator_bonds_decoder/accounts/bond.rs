use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe08030fbb6f66fc4")]
pub struct Bond {
    pub config: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub cpmpe: u64,
    pub bump: u8,
    pub max_stake_wanted: u64,
    pub reserved: [u8; 134],
}
