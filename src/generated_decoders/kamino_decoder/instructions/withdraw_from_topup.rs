

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5fe38adcf05f9671")]
pub struct WithdrawFromTopup{
    pub amount: u64,
}

pub struct WithdrawFromTopupInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub topup_vault: solana_sdk::pubkey::Pubkey,
    pub system: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFromTopup {
    type ArrangedAccounts = WithdrawFromTopupInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let topup_vault = accounts.get(1)?;
        let system = accounts.get(2)?;

        Some(WithdrawFromTopupInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            topup_vault: topup_vault.pubkey,
            system: system.pubkey,
        })
    }
}