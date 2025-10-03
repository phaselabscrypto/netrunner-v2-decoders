use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc4ac985c3cba40e3")]
pub struct PlatformFeeSolWrapUnwrapV2 {
    pub args: PlatformFeeWrapUnwrapArgsV2,
    pub order_id: u64,
}

pub struct PlatformFeeSolWrapUnwrapV2InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub payer_wsol_account: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_account: solana_sdk::pubkey::Pubkey,
    pub commission_sol_account: solana_sdk::pubkey::Pubkey,
    pub commission_wsol_account: solana_sdk::pubkey::Pubkey,
    pub source_token_sa: solana_sdk::pubkey::Pubkey,
    pub destination_token_sa: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlatformFeeSolWrapUnwrapV2 {
    type ArrangedAccounts = PlatformFeeSolWrapUnwrapV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let payer_wsol_account = accounts.get(1)?;
        let wsol_mint = accounts.get(2)?;
        let temp_wsol_account = accounts.get(3)?;
        let commission_sol_account = accounts.get(4)?;
        let commission_wsol_account = accounts.get(5)?;
        let source_token_sa = accounts.get(6)?;
        let destination_token_sa = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;

        Some(PlatformFeeSolWrapUnwrapV2InstructionAccounts {
            payer: payer.pubkey,
            payer_wsol_account: payer_wsol_account.pubkey,
            wsol_mint: wsol_mint.pubkey,
            temp_wsol_account: temp_wsol_account.pubkey,
            commission_sol_account: commission_sol_account.pubkey,
            commission_wsol_account: commission_wsol_account.pubkey,
            source_token_sa: source_token_sa.pubkey,
            destination_token_sa: destination_token_sa.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
