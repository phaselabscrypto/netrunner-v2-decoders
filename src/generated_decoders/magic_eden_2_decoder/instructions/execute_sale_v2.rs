

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5bdc31dfcc8135c1")]
pub struct ExecuteSaleV2{
    pub escrow_payment_bump: u8,
    pub program_as_signer_bump: u8,
    pub buyer_price: u64,
    pub token_size: u64,
    pub buyer_state_expiry: i64,
    pub seller_state_expiry: i64,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: u16,
}

pub struct ExecuteSaleV2InstructionAccounts {
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub escrow_payment_account: solana_sdk::pubkey::Pubkey,
    pub buyer_receipt_token_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub auction_house_treasury: solana_sdk::pubkey::Pubkey,
    pub buyer_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_referral: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub seller_referral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub ata_program: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExecuteSaleV2 {
    type ArrangedAccounts = ExecuteSaleV2InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let buyer = accounts.get(0)?;
        let seller = accounts.get(1)?;
        let notary = accounts.get(2)?;
        let token_account = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let escrow_payment_account = accounts.get(6)?;
        let buyer_receipt_token_account = accounts.get(7)?;
        let authority = accounts.get(8)?;
        let auction_house = accounts.get(9)?;
        let auction_house_treasury = accounts.get(10)?;
        let buyer_trade_state = accounts.get(11)?;
        let buyer_referral = accounts.get(12)?;
        let seller_trade_state = accounts.get(13)?;
        let seller_referral = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let system_program = accounts.get(16)?;
        let ata_program = accounts.get(17)?;
        let program_as_signer = accounts.get(18)?;
        let rent = accounts.get(19)?;

        Some(ExecuteSaleV2InstructionAccounts {
            buyer: buyer.pubkey,
            seller: seller.pubkey,
            notary: notary.pubkey,
            token_account: token_account.pubkey,
            token_mint: token_mint.pubkey,
            metadata: metadata.pubkey,
            escrow_payment_account: escrow_payment_account.pubkey,
            buyer_receipt_token_account: buyer_receipt_token_account.pubkey,
            authority: authority.pubkey,
            auction_house: auction_house.pubkey,
            auction_house_treasury: auction_house_treasury.pubkey,
            buyer_trade_state: buyer_trade_state.pubkey,
            buyer_referral: buyer_referral.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            seller_referral: seller_referral.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            ata_program: ata_program.pubkey,
            program_as_signer: program_as_signer.pubkey,
            rent: rent.pubkey,
        })
    }
}