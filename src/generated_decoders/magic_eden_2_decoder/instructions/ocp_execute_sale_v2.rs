
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc8531f529c9c1461")]
pub struct OcpExecuteSaleV2{
    pub args: OCPExecuteSaleV2Args,
}

pub struct OcpExecuteSaleV2InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub buyer: solana_sdk::pubkey::Pubkey,
    pub seller: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub seller_token_ata: solana_sdk::pubkey::Pubkey,
    pub buyer_token_ata: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub auction_house_treasury: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_trade_state: solana_sdk::pubkey::Pubkey,
    pub buyer_escrow_payment_account: solana_sdk::pubkey::Pubkey,
    pub buyer_referral: solana_sdk::pubkey::Pubkey,
    pub seller_referral: solana_sdk::pubkey::Pubkey,
    pub ocp_mint_state: solana_sdk::pubkey::Pubkey,
    pub ocp_policy: solana_sdk::pubkey::Pubkey,
    pub ocp_freeze_authority: solana_sdk::pubkey::Pubkey,
    pub ocp_program: solana_sdk::pubkey::Pubkey,
    pub cmt_program: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OcpExecuteSaleV2 {
    type ArrangedAccounts = OcpExecuteSaleV2InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let buyer = accounts.get(1)?;
        let seller = accounts.get(2)?;
        let notary = accounts.get(3)?;
        let program_as_signer = accounts.get(4)?;
        let seller_token_ata = accounts.get(5)?;
        let buyer_token_ata = accounts.get(6)?;
        let token_mint = accounts.get(7)?;
        let metadata = accounts.get(8)?;
        let auction_house = accounts.get(9)?;
        let auction_house_treasury = accounts.get(10)?;
        let seller_trade_state = accounts.get(11)?;
        let buyer_trade_state = accounts.get(12)?;
        let buyer_escrow_payment_account = accounts.get(13)?;
        let buyer_referral = accounts.get(14)?;
        let seller_referral = accounts.get(15)?;
        let ocp_mint_state = accounts.get(16)?;
        let ocp_policy = accounts.get(17)?;
        let ocp_freeze_authority = accounts.get(18)?;
        let ocp_program = accounts.get(19)?;
        let cmt_program = accounts.get(20)?;
        let instructions = accounts.get(21)?;
        let associated_token_program = accounts.get(22)?;
        let token_program = accounts.get(23)?;
        let system_program = accounts.get(24)?;
        let rent = accounts.get(25)?;

        Some(OcpExecuteSaleV2InstructionAccounts {
            payer: payer.pubkey,
            buyer: buyer.pubkey,
            seller: seller.pubkey,
            notary: notary.pubkey,
            program_as_signer: program_as_signer.pubkey,
            seller_token_ata: seller_token_ata.pubkey,
            buyer_token_ata: buyer_token_ata.pubkey,
            token_mint: token_mint.pubkey,
            metadata: metadata.pubkey,
            auction_house: auction_house.pubkey,
            auction_house_treasury: auction_house_treasury.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            buyer_trade_state: buyer_trade_state.pubkey,
            buyer_escrow_payment_account: buyer_escrow_payment_account.pubkey,
            buyer_referral: buyer_referral.pubkey,
            seller_referral: seller_referral.pubkey,
            ocp_mint_state: ocp_mint_state.pubkey,
            ocp_policy: ocp_policy.pubkey,
            ocp_freeze_authority: ocp_freeze_authority.pubkey,
            ocp_program: ocp_program.pubkey,
            cmt_program: cmt_program.pubkey,
            instructions: instructions.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}