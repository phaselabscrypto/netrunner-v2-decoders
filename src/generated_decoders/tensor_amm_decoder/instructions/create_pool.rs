use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe992d18ecf6840bc")]
pub struct CreatePool {
    pub args: CreatePoolArgs,
}

pub struct CreatePoolInstructionAccounts {
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub whitelist: solana_sdk::pubkey::Pubkey,
    pub shared_escrow: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePool {
    type ArrangedAccounts = CreatePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let rent_payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let whitelist = accounts.get(3)?;
        let shared_escrow = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(CreatePoolInstructionAccounts {
            rent_payer: rent_payer.pubkey,
            owner: owner.pubkey,
            pool: pool.pubkey,
            whitelist: whitelist.pubkey,
            shared_escrow: shared_escrow.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
