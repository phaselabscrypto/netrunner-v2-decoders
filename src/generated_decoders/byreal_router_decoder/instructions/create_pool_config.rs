use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23a980c2bfe2c606")]
pub struct CreatePoolConfig {
    pub params: CreatePoolConfigParams,
}

pub struct CreatePoolConfigInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub pool_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePoolConfig {
    type ArrangedAccounts = CreatePoolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let pool_id = accounts.get(1)?;
        let pool_config = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(CreatePoolConfigInstructionAccounts {
            admin: admin.pubkey,
            pool_id: pool_id.pubkey,
            pool_config: pool_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
