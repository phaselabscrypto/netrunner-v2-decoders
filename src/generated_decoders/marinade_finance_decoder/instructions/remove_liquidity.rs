use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5055d14818ceb16c")]
pub struct RemoveLiquidity {
    pub tokens: u64,
}

pub struct RemoveLiquidityInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub burn_from: solana_sdk::pubkey::Pubkey,
    pub burn_from_authority: solana_sdk::pubkey::Pubkey,
    pub transfer_sol_to: solana_sdk::pubkey::Pubkey,
    pub transfer_msol_to: solana_sdk::pubkey::Pubkey,
    pub liq_pool_sol_leg_pda: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg: solana_sdk::pubkey::Pubkey,
    pub liq_pool_msol_leg_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveLiquidity {
    type ArrangedAccounts = RemoveLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let lp_mint = accounts.get(1)?;
        let burn_from = accounts.get(2)?;
        let burn_from_authority = accounts.get(3)?;
        let transfer_sol_to = accounts.get(4)?;
        let transfer_msol_to = accounts.get(5)?;
        let liq_pool_sol_leg_pda = accounts.get(6)?;
        let liq_pool_msol_leg = accounts.get(7)?;
        let liq_pool_msol_leg_authority = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(RemoveLiquidityInstructionAccounts {
            state: state.pubkey,
            lp_mint: lp_mint.pubkey,
            burn_from: burn_from.pubkey,
            burn_from_authority: burn_from_authority.pubkey,
            transfer_sol_to: transfer_sol_to.pubkey,
            transfer_msol_to: transfer_msol_to.pubkey,
            liq_pool_sol_leg_pda: liq_pool_sol_leg_pda.pubkey,
            liq_pool_msol_leg: liq_pool_msol_leg.pubkey,
            liq_pool_msol_leg_authority: liq_pool_msol_leg_authority.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
