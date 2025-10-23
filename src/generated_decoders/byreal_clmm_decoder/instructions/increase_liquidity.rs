use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2e9cf3760dcdfbb2")]
pub struct IncreaseLiquidity {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
}

pub struct IncreaseLiquidityInstructionAccounts {
    pub nft_owner: solana_sdk::pubkey::Pubkey,
    pub nft_account: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_position: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub token_account_0: solana_sdk::pubkey::Pubkey,
    pub token_account_1: solana_sdk::pubkey::Pubkey,
    pub token_vault_0: solana_sdk::pubkey::Pubkey,
    pub token_vault_1: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreaseLiquidity {
    type ArrangedAccounts = IncreaseLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let nft_owner = accounts.get(0)?;
        let nft_account = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let protocol_position = accounts.get(3)?;
        let personal_position = accounts.get(4)?;
        let tick_array_lower = accounts.get(5)?;
        let tick_array_upper = accounts.get(6)?;
        let token_account_0 = accounts.get(7)?;
        let token_account_1 = accounts.get(8)?;
        let token_vault_0 = accounts.get(9)?;
        let token_vault_1 = accounts.get(10)?;
        let token_program = accounts.get(11)?;

        Some(IncreaseLiquidityInstructionAccounts {
            nft_owner: nft_owner.pubkey,
            nft_account: nft_account.pubkey,
            pool_state: pool_state.pubkey,
            protocol_position: protocol_position.pubkey,
            personal_position: personal_position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            token_account_0: token_account_0.pubkey,
            token_account_1: token_account_1.pubkey,
            token_vault_0: token_vault_0.pubkey,
            token_vault_1: token_vault_1.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
