use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x09f15e5bad4aa677")]
pub struct UpdateTreasuryFeeVault {
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

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, global_config, fee_mint, treasury_fee_vault, treasury_fee_vault_authority, token_infos, system_program, rent, token_program, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

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
