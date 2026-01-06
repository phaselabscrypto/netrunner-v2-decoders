use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xccb0344f927936f7")]
pub struct Whitelist {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub is_initialized: bool,
    pub bump: u8,
    pub is_swap_fee_exempt: bool,
    pub is_deposit_fee_exempt: bool,
    pub is_withdrawal_fee_exempt: bool,
    pub buffer: [u8; 3],
    pub pool: solana_sdk::pubkey::Pubkey,
}
