use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa45482bd6f3afac8")]
pub struct UpdateGlobalConfig {
    pub key: u16,
    pub index: u16,
    pub value: [u8; 32],
}

pub struct UpdateGlobalConfigInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateGlobalConfig {
    type ArrangedAccounts = UpdateGlobalConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, global_config, system_program, _remaining @ ..] = accounts.as_slice() else {
            return None;
        };

        Some(UpdateGlobalConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
