use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09dc3f608bc9fd9e")]
pub struct BurnAndClaim {
    pub params: BurnAndClaimParams,
}

pub struct BurnAndClaimInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_token_account: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_vault_token_account: solana_sdk::pubkey::Pubkey,
    pub metadata_account: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub token_record: solana_sdk::pubkey::Pubkey,
    pub trading_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub nft_token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
    pub receiving_token_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnAndClaim {
    type ArrangedAccounts = BurnAndClaimInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_token_account, perpetuals, token_vault, token_vault_token_account, metadata_account, collection_metadata, edition, token_record, trading_account, transfer_authority, metadata_program, nft_mint, nft_token_account, system_program, token_program, ix_sysvar, event_authority, program, receiving_token_mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(BurnAndClaimInstructionAccounts {
            owner: owner.pubkey,
            receiving_token_account: receiving_token_account.pubkey,
            perpetuals: perpetuals.pubkey,
            token_vault: token_vault.pubkey,
            token_vault_token_account: token_vault_token_account.pubkey,
            metadata_account: metadata_account.pubkey,
            collection_metadata: collection_metadata.pubkey,
            edition: edition.pubkey,
            token_record: token_record.pubkey,
            trading_account: trading_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            metadata_program: metadata_program.pubkey,
            nft_mint: nft_mint.pubkey,
            nft_token_account: nft_token_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
            receiving_token_mint: receiving_token_mint.pubkey,
        })
    }
}
