use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xd867e7f6ab637c85")]
pub struct SettlementClaim {
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub stake_account_to: solana_sdk::pubkey::Pubkey,
    pub stake_account_staker: solana_sdk::pubkey::Pubkey,
    pub stake_account_withdrawer: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub bump: u8,
    pub rent_collector: solana_sdk::pubkey::Pubkey,
    pub reserved: [u8; 93],
}
