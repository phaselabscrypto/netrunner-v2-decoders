use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8006e48337a134a9")]
pub struct EnableOrDisablePool {
    pub enable: bool,
}

pub struct EnableOrDisablePoolInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EnableOrDisablePool {
    type ArrangedAccounts = EnableOrDisablePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let admin = accounts.get(1)?;

        Some(EnableOrDisablePoolInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}
