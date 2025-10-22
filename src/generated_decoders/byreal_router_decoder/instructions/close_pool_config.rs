use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3deb3b61255d48a1")]
pub struct ClosePoolConfig {}

pub struct ClosePoolConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub pool_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePoolConfig {
    type ArrangedAccounts = ClosePoolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let pool_config = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(ClosePoolConfigInstructionAccounts {
            admin: admin.pubkey,
            pool_config: pool_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
