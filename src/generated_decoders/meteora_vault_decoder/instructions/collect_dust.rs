use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6951552a04afef0")]
pub struct CollectDust {}

pub struct CollectDustInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_admin: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectDust {
    type ArrangedAccounts = CollectDustInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let token_vault = accounts.get(1)?;
        let token_admin = accounts.get(2)?;
        let admin = accounts.get(3)?;
        let token_program = accounts.get(4)?;

        Some(CollectDustInstructionAccounts {
            vault: vault.pubkey,
            token_vault: token_vault.pubkey,
            token_admin: token_admin.pubkey,
            admin: admin.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
