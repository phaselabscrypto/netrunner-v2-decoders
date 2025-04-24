use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb59d59438fb63448")]
pub struct AddLiquidity {
    pub amount: u64,
}

pub struct AddLiquidityInstructionAccounts {
    pub from: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub mint_lp_tokens_to: solana_sdk::pubkey::Pubkey,
    pub flash_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidity {
    type ArrangedAccounts = AddLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let from = accounts.get(0)?;
        let pool_account = accounts.get(1)?;
        let pool_sol_reserves = accounts.get(2)?;
        let lp_mint = accounts.get(3)?;
        let mint_lp_tokens_to = accounts.get(4)?;
        let flash_account = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(AddLiquidityInstructionAccounts {
            from: from.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            lp_mint: lp_mint.pubkey,
            mint_lp_tokens_to: mint_lp_tokens_to.pubkey,
            flash_account: flash_account.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
