use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x51d9b14128e308a5")]
pub struct UpdateStrategyConfig {
    pub mode: u16,
    pub value: [u8; 128],
}

pub struct UpdateStrategyConfigInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub new_account: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateStrategyConfig {
    type ArrangedAccounts = UpdateStrategyConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, new_account, strategy, global_config, system_program, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(UpdateStrategyConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            new_account: new_account.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
