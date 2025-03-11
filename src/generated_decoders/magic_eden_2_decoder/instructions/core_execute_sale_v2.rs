
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd562c518f2359a23")]
pub struct CoreExecuteSaleV2{
    pub args: CoreExecuteSaleV2Args,
}

pub struct CoreExecuteSaleV2InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub asset: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub auction_house_treasury: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_escrow_payment_account: solana_sdk::pubkey::Pubkey,
    pub buyer_referral: solana_sdk::pubkey::Pubkey,
    pub seller_referral: solana_sdk::pubkey::Pubkey,
    pub asset_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payment_mint: solana_sdk::pubkey::Pubkey,
    pub payment_source_token_account: solana_sdk::pubkey::Pubkey,
    pub payment_seller_token_account: solana_sdk::pubkey::Pubkey,
    pub payment_treasury_token_account: solana_sdk::pubkey::Pubkey,
    pub payment_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CoreExecuteSaleV2 {
    type ArrangedAccounts = CoreExecuteSaleV2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let buyer = accounts.get(1)?;
        let seller = accounts.get(2)?;
        let notary = accounts.get(3)?;
        let program_as_signer = accounts.get(4)?;
        let asset = accounts.get(5)?;
        let auction_house = accounts.get(6)?;
        let auction_house_treasury = accounts.get(7)?;
        let seller_trade_state = accounts.get(8)?;
        let buyer_trade_state = accounts.get(9)?;
        let buyer_escrow_payment_account = accounts.get(10)?;
        let buyer_referral = accounts.get(11)?;
        let seller_referral = accounts.get(12)?;
        let asset_program = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let collection = accounts.get(15)?;
        let payment_mint = accounts.get(16)?;
        let payment_source_token_account = accounts.get(17)?;
        let payment_seller_token_account = accounts.get(18)?;
        let payment_treasury_token_account = accounts.get(19)?;
        let payment_token_program = accounts.get(20)?;

        Some(CoreExecuteSaleV2InstructionAccounts {
            payer: payer.pubkey,
            buyer: buyer.pubkey,
            seller: seller.pubkey,
            notary: notary.pubkey,
            program_as_signer: program_as_signer.pubkey,
            asset: asset.pubkey,
            auction_house: auction_house.pubkey,
            auction_house_treasury: auction_house_treasury.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            buyer_trade_state: buyer_trade_state.pubkey,
            buyer_escrow_payment_account: buyer_escrow_payment_account.pubkey,
            buyer_referral: buyer_referral.pubkey,
            seller_referral: seller_referral.pubkey,
            asset_program: asset_program.pubkey,
            system_program: system_program.pubkey,
            collection: collection.pubkey,
            payment_mint: payment_mint.pubkey,
            payment_source_token_account: payment_source_token_account.pubkey,
            payment_seller_token_account: payment_seller_token_account.pubkey,
            payment_treasury_token_account: payment_treasury_token_account.pubkey,
            payment_token_program: payment_token_program.pubkey,
        })
    }
}