use super::super::types::SwapSpecInput;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x56b7a390003aad1c")]
pub struct SwapRoute {
    pub amount: u64,
    pub minimum_amount_out: u64,
    pub mints: u8,
    pub provider_fee_bps: u16,
    pub service_fee_bps: u16,
    pub swaps: Vec<SwapSpecInput>,
}

pub struct SwapRouteInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub atlas: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub output_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapRoute {
    type ArrangedAccounts = SwapRouteInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let atlas = accounts.get(1)?;
        let input_mint = accounts.get(2)?;
        let input_token_account = accounts.get(3)?;
        let output_mint = accounts.get(4)?;
        let output_token_account = accounts.get(5)?;
        let input_token_program = accounts.get(6)?;
        let output_token_program = accounts.get(7)?;

        Some(SwapRouteInstructionAccounts {
            payer: payer.pubkey,
            atlas: atlas.pubkey,
            input_mint: input_mint.pubkey,
            input_token_account: input_token_account.pubkey,
            output_mint: output_mint.pubkey,
            output_token_account: output_token_account.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
        })
    }
}
