use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xc3372e293607e19b")]
pub struct BuyerTradeStateV2 {
    pub auction_house_key: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub buyer_referral: solana_sdk::pubkey::Pubkey,
    pub buyer_price: u64,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub token_size: u64,
    pub bump: u8,
    pub expiry: i64,
    pub buyer_creator_royalty_bp: u16,
    pub payment_mint: solana_sdk::pubkey::Pubkey,
}
