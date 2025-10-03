use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x132c829448382cee")]
pub struct ProxySwap {
    pub data: SwapArgs,
    pub order_id: u64,
}

pub struct ProxySwapInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub sa_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_sa: solana_sdk::pubkey::Pubkey,
    pub destination_token_sa: solana_sdk::pubkey::Pubkey,
    pub source_token_program: solana_sdk::pubkey::Pubkey,
    pub destination_token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProxySwap {
    type ArrangedAccounts = ProxySwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let source_token_account = accounts.get(1)?;
        let destination_token_account = accounts.get(2)?;
        let source_mint = accounts.get(3)?;
        let destination_mint = accounts.get(4)?;
        let sa_authority = accounts.get(5)?;
        let source_token_sa = accounts.get(6)?;
        let destination_token_sa = accounts.get(7)?;
        let source_token_program = accounts.get(8)?;
        let destination_token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;

        Some(ProxySwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            sa_authority: sa_authority.pubkey,
            source_token_sa: source_token_sa.pubkey,
            destination_token_sa: destination_token_sa.pubkey,
            source_token_program: source_token_program.pubkey,
            destination_token_program: destination_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
