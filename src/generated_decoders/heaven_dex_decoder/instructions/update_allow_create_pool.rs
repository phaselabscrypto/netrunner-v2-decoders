use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdffce73e60dbf1d6")]
pub struct UpdateAllowCreatePool {
    pub version: u16,
    pub allow_create_pool: bool,
}

pub struct UpdateAllowCreatePoolInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub protocol_config_state: solana_sdk::pubkey::Pubkey,
    pub protocol_admin_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAllowCreatePool {
    type ArrangedAccounts = UpdateAllowCreatePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let protocol_config_state = accounts.get(1)?;
        let protocol_admin_state = accounts.get(2)?;

        Some(UpdateAllowCreatePoolInstructionAccounts {
            admin: admin.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            protocol_admin_state: protocol_admin_state.pubkey,
        })
    }
}
