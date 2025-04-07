
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe992d18ecf6840bc")]
pub struct CreatePool{
    pub fee: Fee,
}

pub struct CreatePoolInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub fee_account: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePool {
    type ArrangedAccounts = CreatePoolInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let pool_account = accounts.get(2)?;
        let pool_sol_reserves = accounts.get(3)?;
        let fee_account = accounts.get(4)?;
        let lp_mint = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;

        Some(CreatePoolInstructionAccounts {
            payer: payer.pubkey,
            fee_authority: fee_authority.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            fee_account: fee_account.pubkey,
            lp_mint: lp_mint.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}