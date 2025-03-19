use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00a4564c38480caa")]
pub struct WithdrawFromTreasury {
    pub amount: u64,
}

pub struct WithdrawFromTreasuryInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFromTreasury {
    type ArrangedAccounts = WithdrawFromTreasuryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, global_config, mint, treasury_fee_vault, treasury_fee_vault_authority, token_account, system_program, rent, token_program, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(WithdrawFromTreasuryInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            mint: mint.pubkey,
            treasury_fee_vault: treasury_fee_vault.pubkey,
            treasury_fee_vault_authority: treasury_fee_vault_authority.pubkey,
            token_account: token_account.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
