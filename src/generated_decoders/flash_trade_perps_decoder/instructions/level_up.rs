use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8040c574e28177ea")]
pub struct LevelUp {
    pub params: LevelUpParams,
}

pub struct LevelUpInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub metadata_account: solana_sdk::pubkey::Pubkey,
    pub trading_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub nft_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_program: solana_sdk::pubkey::Pubkey,
    pub authorization_rules_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LevelUp {
    type ArrangedAccounts = LevelUpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, perpetuals, pool, metadata_account, trading_account, transfer_authority, metadata_program, nft_mint, system_program, ix_sysvar, authorization_rules_program, authorization_rules_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LevelUpInstructionAccounts {
            owner: owner.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            metadata_account: metadata_account.pubkey,
            trading_account: trading_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            metadata_program: metadata_program.pubkey,
            nft_mint: nft_mint.pubkey,
            system_program: system_program.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
            authorization_rules_program: authorization_rules_program.pubkey,
            authorization_rules_account: authorization_rules_account.pubkey,
        })
    }
}
