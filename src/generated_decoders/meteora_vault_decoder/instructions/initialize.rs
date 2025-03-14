

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize{
}

pub struct InitializeInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let token_vault = accounts.get(2)?;
        let token_mint = accounts.get(3)?;
        let lp_mint = accounts.get(4)?;
        let rent = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(InitializeInstructionAccounts {
            vault: vault.pubkey,
            payer: payer.pubkey,
            token_vault: token_vault.pubkey,
            token_mint: token_mint.pubkey,
            lp_mint: lp_mint.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}