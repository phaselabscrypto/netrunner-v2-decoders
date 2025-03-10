

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x963e7ddbabdc1aed")]
pub struct UpdateActivationPoint{
    pub new_activation_point: u64,
}

pub struct UpdateActivationPointInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateActivationPoint {
    type ArrangedAccounts = UpdateActivationPointInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let admin = accounts.get(1)?;

        Some(UpdateActivationPointInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}