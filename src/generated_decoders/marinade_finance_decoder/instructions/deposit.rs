

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit{
    pub lamports: u64,
}

pub struct DepositInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_pda: solana_sdk::pubkey::Pubkey,
    pub transfer_from: solana_sdk::pubkey::Pubkey,
    pub mint_to: solana_sdk::pubkey::Pubkey,
    pub msol_mint_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let msol_mint = accounts.get(1)?;
        let liq_pool_sol_leg_pda = accounts.get(2)?;
        let liq_pool_msol_leg = accounts.get(3)?;
        let liq_pool_msol_leg_authority = accounts.get(4)?;
        let reserve_pda = accounts.get(5)?;
        let transfer_from = accounts.get(6)?;
        let mint_to = accounts.get(7)?;
        let msol_mint_authority = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(DepositInstructionAccounts {
            state: state.pubkey,
            msol_mint: msol_mint.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            liq_pool_msol_leg_authority: liq_pool_msol_leg_authority.pubkey,
            reserve_pda: reserve_pda.pubkey,
            transfer_from: transfer_from.pubkey,
            mint_to: mint_to.pubkey,
            msol_mint_authority: msol_mint_authority.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}