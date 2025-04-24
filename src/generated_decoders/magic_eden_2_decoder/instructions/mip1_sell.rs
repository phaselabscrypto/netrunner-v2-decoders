use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3a32ac6fa697165e")]
pub struct Mip1Sell {
    pub args: MIP1SellArgs,
}

pub struct Mip1SellInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub notary: solana_sdk::pubkey::Pubkey,
    pub program_as_signer: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub auction_house: solana_sdk::pubkey::Pubkey,
    pub seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub migration_seller_trade_state: solana_sdk::pubkey::Pubkey,
    pub seller_referral: solana_sdk::pubkey::Pubkey,
    pub token_ata: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub destination_token_record: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Mip1Sell {
    type ArrangedAccounts = Mip1SellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let notary = accounts.get(1)?;
        let program_as_signer = accounts.get(2)?;
        let token_account = accounts.get(3)?;
        let token_mint = accounts.get(4)?;
        let metadata = accounts.get(5)?;
        let auction_house = accounts.get(6)?;
        let seller_trade_state = accounts.get(7)?;
        let migration_seller_trade_state = accounts.get(8)?;
        let seller_referral = accounts.get(9)?;
        let token_ata = accounts.get(10)?;
        let token_metadata_program = accounts.get(11)?;
        let edition = accounts.get(12)?;
        let authorization_rules_program = accounts.get(13)?;
        let authorization_rules = accounts.get(14)?;
        let instructions = accounts.get(15)?;
        let owner_token_record = accounts.get(16)?;
        let destination_token_record = accounts.get(17)?;
        let associated_token_program = accounts.get(18)?;
        let token_program = accounts.get(19)?;
        let system_program = accounts.get(20)?;
        let rent = accounts.get(21)?;

        Some(Mip1SellInstructionAccounts {
            wallet: wallet.pubkey,
            notary: notary.pubkey,
            program_as_signer: program_as_signer.pubkey,
            token_account: token_account.pubkey,
            token_mint: token_mint.pubkey,
            metadata: metadata.pubkey,
            auction_house: auction_house.pubkey,
            seller_trade_state: seller_trade_state.pubkey,
            migration_seller_trade_state: migration_seller_trade_state.pubkey,
            seller_referral: seller_referral.pubkey,
            token_ata: token_ata.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            edition: edition.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules: authorization_rules.pubkey,
            instructions: instructions.pubkey,
            owner_token_record: owner_token_record.pubkey,
            destination_token_record: destination_token_record.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
