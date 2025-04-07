

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb59d59438fb63448")]
pub struct AddLiquidity{
    pub lamports: u64,
}

pub struct AddLiquidityInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub lp_mint_authority: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub transfer_from: solana_sdk::pubkey::Pubkey,
    pub mint_to: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidity {
    type ArrangedAccounts = AddLiquidityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let lp_mint_authority = accounts.get(2)?;
        let liq_pool_msol_leg = accounts.get(3)?;
        let liq_pool_sol_leg_pda = accounts.get(4)?;
        let transfer_from = accounts.get(5)?;
        let mint_to = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;

        Some(AddLiquidityInstructionAccounts {
            state: state.pubkey,
            lp_mint: lp_mint.pubkey,
            lp_mint_authority: lp_mint_authority.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            transfer_from: transfer_from.pubkey,
            mint_to: mint_to.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}