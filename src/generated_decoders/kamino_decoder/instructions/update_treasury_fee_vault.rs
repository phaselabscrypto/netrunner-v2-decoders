

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x09f15e5bad4aa677")]
pub struct UpdateTreasuryFeeVault{
    pub collateral_id: u16,
}

pub struct UpdateTreasuryFeeVaultInstructionAccounts {
    pub signer: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub fee_mint: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateTreasuryFeeVault {
    type ArrangedAccounts = UpdateTreasuryFeeVaultInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let signer = accounts.get(0)?;
        let global_config = accounts.get(1)?;
        let fee_mint = accounts.get(2)?;
        let treasury_fee_vault = accounts.get(3)?;
        let treasury_fee_vault_authority = accounts.get(4)?;
        let token_infos = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;
        let token_program = accounts.get(8)?;

        Some(UpdateTreasuryFeeVaultInstructionAccounts {
            signer: signer.pubkey,
            global_config: global_config.pubkey,
            fee_mint: fee_mint.pubkey,
            treasury_fee_vault: treasury_fee_vault.pubkey,
            treasury_fee_vault_authority: treasury_fee_vault_authority.pubkey,
            token_infos: token_infos.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
        })
    }
}