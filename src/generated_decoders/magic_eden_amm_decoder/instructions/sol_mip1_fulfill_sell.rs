
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3b0b496b286940d2")]
pub struct SolMip1FulfillSell{
    pub args: SolMip1FulfillSellArgs,
}

pub struct SolMip1FulfillSellInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub asset_metadata: solana_sdk::pubkey::Pubkey,
    pub asset_mint: solana_sdk::pubkey::Pubkey,
    pub asset_master_edition: solana_sdk::pubkey::Pubkey,
    pub sellside_escrow_token_account: solana_sdk::pubkey::Pubkey,
    pub payer_asset_account: solana_sdk::pubkey::Pubkey,
    pub allowlist_aux_account: solana_sdk::pubkey::Pubkey,
    pub sell_state: solana_sdk::pubkey::Pubkey,
    pub owner_token_record: solana_sdk::pubkey::Pubkey,
    pub destination_token_record: solana_sdk::pubkey::Pubkey,
    pub authorization_rules: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub instructions: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SolMip1FulfillSell {
    type ArrangedAccounts = SolMip1FulfillSellInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let cosigner = accounts.get(2)?;
        let referral = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let buyside_sol_escrow_account = accounts.get(5)?;
        let asset_metadata = accounts.get(6)?;
        let asset_mint = accounts.get(7)?;
        let asset_master_edition = accounts.get(8)?;
        let sellside_escrow_token_account = accounts.get(9)?;
        let payer_asset_account = accounts.get(10)?;
        let allowlist_aux_account = accounts.get(11)?;
        let sell_state = accounts.get(12)?;
        let owner_token_record = accounts.get(13)?;
        let destination_token_record = accounts.get(14)?;
        let authorization_rules = accounts.get(15)?;
        let token_metadata_program = accounts.get(16)?;
        let authorization_rules_program = accounts.get(17)?;
        let instructions = accounts.get(18)?;
        let system_program = accounts.get(19)?;
        let token_program = accounts.get(20)?;
        let associated_token_program = accounts.get(21)?;
        let rent = accounts.get(22)?;

        Some(SolMip1FulfillSellInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            referral: referral.pubkey,
            pool: pool.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            asset_metadata: asset_metadata.pubkey,
            asset_mint: asset_mint.pubkey,
            asset_master_edition: asset_master_edition.pubkey,
            sellside_escrow_token_account: sellside_escrow_token_account.pubkey,
            payer_asset_account: payer_asset_account.pubkey,
            allowlist_aux_account: allowlist_aux_account.pubkey,
            sell_state: sell_state.pubkey,
            owner_token_record: owner_token_record.pubkey,
            destination_token_record: destination_token_record.pubkey,
            authorization_rules: authorization_rules.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            instructions: instructions.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}