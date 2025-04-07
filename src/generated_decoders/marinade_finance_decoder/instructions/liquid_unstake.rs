

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1e1e77f0bfe30c10")]
pub struct LiquidUnstake{
    pub msol_amount: u64,
}

pub struct LiquidUnstakeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub treasury_msol_account: solana_sdk::pubkey::Pubkey,
    pub get_msol_from: solana_sdk::pubkey::Pubkey,
    pub get_msol_from_authority: solana_sdk::pubkey::Pubkey,
    pub transfer_sol_to: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidUnstake {
    type ArrangedAccounts = LiquidUnstakeInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let msol_mint = accounts.get(1)?;
        let liq_pool_sol_leg_pda = accounts.get(2)?;
        let liq_pool_msol_leg = accounts.get(3)?;
        let treasury_msol_account = accounts.get(4)?;
        let get_msol_from = accounts.get(5)?;
        let get_msol_from_authority = accounts.get(6)?;
        let transfer_sol_to = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;

        Some(LiquidUnstakeInstructionAccounts {
            state: state.pubkey,
            msol_mint: msol_mint.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            treasury_msol_account: treasury_msol_account.pubkey,
            get_msol_from: get_msol_from.pubkey,
            get_msol_from_authority: get_msol_from_authority.pubkey,
            transfer_sol_to: transfer_sol_to.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}