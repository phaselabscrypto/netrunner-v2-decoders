use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa78a4e95dfc2067e")]
pub struct CollectFundFee {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

pub struct CollectFundFeeInstructionAccounts {
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub token_vault_0: solana_sdk::pubkey::Pubkey,
    pub token_vault_1: solana_sdk::pubkey::Pubkey,
    pub vault_0_mint: solana_sdk::pubkey::Pubkey,
    pub vault_1_mint: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account_0: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account_1: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program_2022: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFundFee {
    type ArrangedAccounts = CollectFundFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin_group = accounts.get(0)?;
        let pool_state = accounts.get(1)?;
        let token_vault_0 = accounts.get(2)?;
        let token_vault_1 = accounts.get(3)?;
        let vault_0_mint = accounts.get(4)?;
        let vault_1_mint = accounts.get(5)?;
        let recipient_token_account_0 = accounts.get(6)?;
        let recipient_token_account_1 = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let token_program_2022 = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;

        Some(CollectFundFeeInstructionAccounts {
            admin_group: admin_group.pubkey,
            pool_state: pool_state.pubkey,
            token_vault_0: token_vault_0.pubkey,
            token_vault_1: token_vault_1.pubkey,
            vault_0_mint: vault_0_mint.pubkey,
            vault_1_mint: vault_1_mint.pubkey,
            recipient_token_account_0: recipient_token_account_0.pubkey,
            recipient_token_account_1: recipient_token_account_1.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
