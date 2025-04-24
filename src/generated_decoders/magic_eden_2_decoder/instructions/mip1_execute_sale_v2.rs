use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeca3ccad4790eb76")]
pub struct Mip1ExecuteSaleV2 {
    pub args: MIP1ExecuteSaleV2Args,
}

pub struct Mip1ExecuteSaleV2InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub buyer_receipt_token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub auction_house_treasury: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_escrow_payment_account: solana_sdk::pubkey::Pubkey,
    pub buyer_referral: solana_sdk::pubkey::Pubkey,
    pub seller_referral: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub destination_token_record: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Mip1ExecuteSaleV2 {
    type ArrangedAccounts = Mip1ExecuteSaleV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let buyer = accounts.get(1)?;
        let seller = accounts.get(2)?;
        let notary = accounts.get(3)?;
        let program_as_signer = accounts.get(4)?;
        let token_account = accounts.get(5)?;
        let buyer_receipt_token_account = accounts.get(6)?;
        let token_mint = accounts.get(7)?;
        let metadata = accounts.get(8)?;
        let auction_house = accounts.get(9)?;
        let auction_house_treasury = accounts.get(10)?;
        let seller_trade_state = accounts.get(11)?;
        let buyer_trade_state = accounts.get(12)?;
        let buyer_escrow_payment_account = accounts.get(13)?;
        let buyer_referral = accounts.get(14)?;
        let seller_referral = accounts.get(15)?;
        let token_metadata_program = accounts.get(16)?;
        let edition = accounts.get(17)?;
        let authorization_rules_program = accounts.get(18)?;
        let authorization_rules = accounts.get(19)?;
        let owner_token_record = accounts.get(20)?;
        let destination_token_record = accounts.get(21)?;
        let instructions = accounts.get(22)?;
        let associated_token_program = accounts.get(23)?;
        let token_program = accounts.get(24)?;
        let system_program = accounts.get(25)?;
        let rent = accounts.get(26)?;

        Some(Mip1ExecuteSaleV2InstructionAccounts {
            payer: payer.pubkey,
            buyer: buyer.pubkey,
            seller: seller.pubkey,
            notary: notary.pubkey,
            program_as_signer: program_as_signer.pubkey,
            token_account: token_account.pubkey,
            buyer_receipt_token_account: buyer_receipt_token_account.pubkey,
            token_mint: token_mint.pubkey,
            metadata: metadata.pubkey,
            auction_house: auction_house.pubkey,
            auction_house_treasury: auction_house_treasury.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            buyer_trade_state: buyer_trade_state.pubkey,
            buyer_escrow_payment_account: buyer_escrow_payment_account.pubkey,
            buyer_referral: buyer_referral.pubkey,
            seller_referral: seller_referral.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            edition: edition.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
            owner_token_record: owner_token_record.pubkey,
            destination_token_record: destination_token_record.pubkey,
            instructions: instructions.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
