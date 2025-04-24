use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8cb936ac0f5e1f9b")]
pub struct InitUpdateTswap {
    pub config: TSwapConfig,
}

pub struct InitUpdateTswapInstructionAccounts {
    pub tswap: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub new_owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitUpdateTswap {
    type ArrangedAccounts = InitUpdateTswapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tswap = accounts.get(0)?;
        let fee_vault = accounts.get(1)?;
        let cosigner = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let new_owner = accounts.get(5)?;

        Some(InitUpdateTswapInstructionAccounts {
            tswap: tswap.pubkey,
            fee_vault: fee_vault.pubkey,
            cosigner: cosigner.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
            new_owner: new_owner.pubkey,
        })
    }
}
