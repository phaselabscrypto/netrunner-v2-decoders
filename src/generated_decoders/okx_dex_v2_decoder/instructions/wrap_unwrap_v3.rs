use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb4b2bf3646080de0")]
pub struct WrapUnwrapV3 {
    pub args: PlatformFeeWrapUnwrapArgs,
}

pub struct WrapUnwrapV3InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub payer_wsol_account: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_account: solana_sdk::pubkey::Pubkey,
    pub commission_account: solana_sdk::pubkey::Pubkey,
    pub platform_fee_account: solana_sdk::pubkey::Pubkey,
    pub authority_pda: solana_sdk::pubkey::Pubkey,
    pub wsol_sa: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WrapUnwrapV3 {
    type ArrangedAccounts = WrapUnwrapV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let payer_wsol_account = accounts.get(1)?;
        let wsol_mint = accounts.get(2)?;
        let temp_wsol_account = accounts.get(3)?;
        let commission_account = accounts.get(4)?;
        let platform_fee_account = accounts.get(5)?;
        let authority_pda = accounts.get(6)?;
        let wsol_sa = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let system_program = accounts.get(9)?;

        Some(WrapUnwrapV3InstructionAccounts {
            payer: payer.pubkey,
            payer_wsol_account: payer_wsol_account.pubkey,
            wsol_mint: wsol_mint.pubkey,
            temp_wsol_account: temp_wsol_account.pubkey,
            commission_account: commission_account.pubkey,
            platform_fee_account: platform_fee_account.pubkey,
            authority_pda: authority_pda.pubkey,
            wsol_sa: wsol_sa.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
