use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x86addfb94d561c33")]
pub struct Order {
    pub closer: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub return_input_token_account: solana_sdk::pubkey::Pubkey,
    pub return_rent_to: solana_sdk::pubkey::Pubkey,
    pub id: u64,
    pub quoted_out_amount: u64,
    pub last_fillable_slot: u64,
    pub slippage_bps: u16,
    pub bump: u8,
    pub vault_bump: u8,
    pub flags: u8,
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
}
