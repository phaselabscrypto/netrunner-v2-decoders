use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0de3a4ec202706ff")]
pub struct UpdateStrategyAdmin {}

pub struct UpdateStrategyAdminInstructionAccounts {
    pub pending_admin: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateStrategyAdmin {
    type ArrangedAccounts = UpdateStrategyAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let pending_admin = accounts.get(0)?;
        let strategy = accounts.get(1)?;

        Some(UpdateStrategyAdminInstructionAccounts {
            pending_admin: pending_admin.pubkey,
            strategy: strategy.pubkey,
        })
    }
}
