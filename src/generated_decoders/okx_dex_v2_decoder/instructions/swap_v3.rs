use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf0e02621b01ff1af")]
pub struct SwapV3 {
    pub args: SwapArgs,
    pub commission_info: u32,
    pub platform_fee_rate: u16,
    pub order_id: u64,
}

pub struct SwapV3InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub commission_account: solana_sdk::pubkey::Pubkey,
    pub platform_fee_account: solana_sdk::pubkey::Pubkey,
    pub sa_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_sa: solana_sdk::pubkey::Pubkey,
    pub destination_token_sa: solana_sdk::pubkey::Pubkey,
    pub source_token_program: solana_sdk::pubkey::Pubkey,
    pub destination_token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapV3 {
    type ArrangedAccounts = SwapV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let source_token_account = accounts.get(1)?;
        let destination_token_account = accounts.get(2)?;
        let source_mint = accounts.get(3)?;
        let destination_mint = accounts.get(4)?;
        let commission_account = accounts.get(5)?;
        let platform_fee_account = accounts.get(6)?;
        let sa_authority = accounts.get(7)?;
        let source_token_sa = accounts.get(8)?;
        let destination_token_sa = accounts.get(9)?;
        let source_token_program = accounts.get(10)?;
        let destination_token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let system_program = accounts.get(13)?;

        Some(SwapV3InstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            commission_account: commission_account.pubkey,
            platform_fee_account: platform_fee_account.pubkey,
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
