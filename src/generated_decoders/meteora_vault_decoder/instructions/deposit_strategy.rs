use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf65239e283defdf9")]
pub struct DepositStrategy {
    pub amount: u64,
}

pub struct DepositStrategyInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub strategy_program: solana_sdk::pubkey::Pubkey,
    pub collateral_vault: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositStrategy {
    type ArrangedAccounts = DepositStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let token_vault = accounts.get(2)?;
        let fee_vault = accounts.get(3)?;
        let lp_mint = accounts.get(4)?;
        let strategy_program = accounts.get(5)?;
        let collateral_vault = accounts.get(6)?;
        let reserve = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let operator = accounts.get(9)?;

        Some(DepositStrategyInstructionAccounts {
            vault: vault.pubkey,
            strategy: strategy.pubkey,
            token_vault: token_vault.pubkey,
            fee_vault: fee_vault.pubkey,
            lp_mint: lp_mint.pubkey,
            strategy_program: strategy_program.pubkey,
            collateral_vault: collateral_vault.pubkey,
            reserve: reserve.pubkey,
            token_program: token_program.pubkey,
            operator: operator.pubkey,
        })
    }
}
