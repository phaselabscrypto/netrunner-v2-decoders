

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb817ee6167c5d33d")]
pub struct BuyV2{
    pub buyer_price: u64,
    pub token_size: u64,
    pub buyer_state_expiry: i64,
    pub buyer_creator_royalty_bp: u16,
    pub extra_args: Vec<u8>,
}

pub struct BuyV2InstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub escrow_payment_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub buyer_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyV2 {
    type ArrangedAccounts = BuyV2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let token_mint = accounts.get(2)?;
        let metadata = accounts.get(3)?;
        let escrow_payment_account = accounts.get(4)?;
        let authority = accounts.get(5)?;
        let auction_house = accounts.get(6)?;
        let buyer_trade_state = accounts.get(7)?;
        let buyer_referral = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let system_program = accounts.get(10)?;

        Some(BuyV2InstructionAccounts {
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            token_mint: token_mint.pubkey,
            metadata: metadata.pubkey,
            escrow_payment_account: escrow_payment_account.pubkey,
            authority: authority.pubkey,
            auction_house: auction_house.pubkey,
            buyer_trade_state: buyer_trade_state.pubkey,
            buyer_referral: buyer_referral.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}