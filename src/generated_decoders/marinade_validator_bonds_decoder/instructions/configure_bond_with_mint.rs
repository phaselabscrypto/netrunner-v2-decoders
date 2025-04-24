use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x30bde6277021e308")]
pub struct ConfigureBondWithMint {
    pub args: ConfigureBondWithMintArgs,
}

pub struct ConfigureBondWithMintInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConfigureBondWithMint {
    type ArrangedAccounts = ConfigureBondWithMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let vote_account = accounts.get(3)?;
        let token_account = accounts.get(4)?;
        let token_authority = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(ConfigureBondWithMintInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            mint: mint.pubkey,
            vote_account: vote_account.pubkey,
            token_account: token_account.pubkey,
            token_authority: token_authority.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
