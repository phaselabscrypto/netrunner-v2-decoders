use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5055d14818ceb16c")]
pub struct RemoveLiquidity {
    pub amount_lp: u64,
}

pub struct RemoveLiquidityInstructionAccounts {
    pub burn_lp_tokens_from_authority: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub burn_lp_tokens_from: solana_sdk::pubkey::Pubkey,
    pub flash_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveLiquidity {
    type ArrangedAccounts = RemoveLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let burn_lp_tokens_from_authority = accounts.get(0)?;
        let to = accounts.get(1)?;
        let pool_account = accounts.get(2)?;
        let pool_sol_reserves = accounts.get(3)?;
        let lp_mint = accounts.get(4)?;
        let burn_lp_tokens_from = accounts.get(5)?;
        let flash_account = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(RemoveLiquidityInstructionAccounts {
            burn_lp_tokens_from_authority: burn_lp_tokens_from_authority.pubkey,
            to: to.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            lp_mint: lp_mint.pubkey,
            burn_lp_tokens_from: burn_lp_tokens_from.pubkey,
            flash_account: flash_account.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
