

use carbon_core::{CarbonDeserialize, borsh};
use serde_with::{serde_as, Bytes};


#[serde_as]
#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x51d9b14128e308a5")]
pub struct UpdateStrategyConfig{
    pub mode: u16,
    #[serde_as(as = "Bytes")]
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

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let new_account = accounts.get(1)?;
        let strategy = accounts.get(2)?;
        let global_config = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(UpdateStrategyConfigInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            new_account: new_account.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}