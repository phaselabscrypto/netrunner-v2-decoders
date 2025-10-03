use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeb47d3c472c78f5c")]
pub struct CommissionSplSwap {
    pub data: CommissionSwapArgs,
    pub order_id: u64,
}

pub struct CommissionSplSwapInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub commission_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CommissionSplSwap {
    type ArrangedAccounts = CommissionSplSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let source_token_account = accounts.get(1)?;
        let destination_token_account = accounts.get(2)?;
        let source_mint = accounts.get(3)?;
        let destination_mint = accounts.get(4)?;
        let commission_token_account = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(CommissionSplSwapInstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            commission_token_account: commission_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
