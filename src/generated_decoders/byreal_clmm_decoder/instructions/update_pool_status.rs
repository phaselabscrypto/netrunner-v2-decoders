use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x82576c062ee0757b")]
pub struct UpdatePoolStatus {
    pub status: u8,
}

pub struct UpdatePoolStatusInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePoolStatus {
    type ArrangedAccounts = UpdatePoolStatusInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let admin_group = accounts.get(1)?;
        let pool_state = accounts.get(2)?;

        Some(UpdatePoolStatusInstructionAccounts {
            authority: authority.pubkey,
            admin_group: admin_group.pubkey,
            pool_state: pool_state.pubkey,
        })
    }
}
