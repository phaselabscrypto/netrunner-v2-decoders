

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9109489d5f7d3d55")]
pub struct CloseConfig{
}

pub struct CloseConfigInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub rent_receiver: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseConfig {
    type ArrangedAccounts = CloseConfigInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let rent_receiver = accounts.get(2)?;

        Some(CloseConfigInstructionAccounts {
            config: config.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
        })
    }
}