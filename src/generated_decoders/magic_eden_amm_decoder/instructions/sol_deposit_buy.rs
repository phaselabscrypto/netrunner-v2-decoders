use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x42e50b366da455ee")]
pub struct SolDepositBuy {
    pub args: SolDepositBuyArgs,
}

pub struct SolDepositBuyInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub cosigner: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub buyside_sol_escrow_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SolDepositBuy {
    type ArrangedAccounts = SolDepositBuyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let cosigner = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let buyside_sol_escrow_account = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(SolDepositBuyInstructionAccounts {
            owner: owner.pubkey,
            cosigner: cosigner.pubkey,
            pool: pool.pubkey,
            buyside_sol_escrow_account: buyside_sol_escrow_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
