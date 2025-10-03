use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x45a4d25992d6ad43")]
pub struct PlatformFeeSplProxySwapV2 {
    pub args: SwapArgs,
    pub commission_info: u32,
    pub platform_fee_rate: u32,
    pub trim_rate: u8,
    pub order_id: u64,
}

pub struct PlatformFeeSplProxySwapV2InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub source_mint: solana_sdk::pubkey::Pubkey,
    pub destination_mint: solana_sdk::pubkey::Pubkey,
    pub commission_token_account: solana_sdk::pubkey::Pubkey,
    pub sa_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_sa: solana_sdk::pubkey::Pubkey,
    pub destination_token_sa: solana_sdk::pubkey::Pubkey,
    pub source_token_program: solana_sdk::pubkey::Pubkey,
    pub destination_token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlatformFeeSplProxySwapV2 {
    type ArrangedAccounts = PlatformFeeSplProxySwapV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let source_token_account = accounts.get(1)?;
        let destination_token_account = accounts.get(2)?;
        let source_mint = accounts.get(3)?;
        let destination_mint = accounts.get(4)?;
        let commission_token_account = accounts.get(5)?;
        let sa_authority = accounts.get(6)?;
        let source_token_sa = accounts.get(7)?;
        let destination_token_sa = accounts.get(8)?;
        let source_token_program = accounts.get(9)?;
        let destination_token_program = accounts.get(10)?;
        let associated_token_program = accounts.get(11)?;
        let system_program = accounts.get(12)?;

        Some(PlatformFeeSplProxySwapV2InstructionAccounts {
            payer: payer.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            commission_token_account: commission_token_account.pubkey,
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
