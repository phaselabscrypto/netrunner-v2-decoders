use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x16b7f99df75f9660")]
pub struct ClaimStatus {
    pub is_claimed: bool,
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub claim_status_payer: solana_sdk::pubkey::Pubkey,
    pub slot_claimed_at: u64,
    pub amount: u64,
    pub expires_at: u64,
    pub bump: u8,
}
