use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x16b7f99df75f9660")]
pub struct ClaimStatus {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub distributor: solana_sdk::pubkey::Pubkey,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub locked_amount: u64,
    pub locked_amount_withdrawn: u64,
    pub unlocked_amount: u64,
    pub bonus_amount: u64,
    pub closable: u8,
    pub padding0: [u8; 7],
    pub padding1: u128,
}
