use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3a7fbc3e4f52c460")]
pub struct DecreaseLiquidityV2 {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

pub struct DecreaseLiquidityV2InstructionAccounts {
    pub nft_owner: solana_sdk::pubkey::Pubkey,
    pub nft_account: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_position: solana_sdk::pubkey::Pubkey,
    pub token_vault_0: solana_sdk::pubkey::Pubkey,
    pub token_vault_1: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account_0: solana_sdk::pubkey::Pubkey,
    pub recipient_token_account_1: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program_2022: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub vault_0_mint: solana_sdk::pubkey::Pubkey,
    pub vault_1_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DecreaseLiquidityV2 {
    type ArrangedAccounts = DecreaseLiquidityV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let nft_owner = accounts.get(0)?;
        let nft_account = accounts.get(1)?;
        let personal_position = accounts.get(2)?;
        let pool_state = accounts.get(3)?;
        let protocol_position = accounts.get(4)?;
        let token_vault_0 = accounts.get(5)?;
        let token_vault_1 = accounts.get(6)?;
        let tick_array_lower = accounts.get(7)?;
        let tick_array_upper = accounts.get(8)?;
        let recipient_token_account_0 = accounts.get(9)?;
        let recipient_token_account_1 = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let token_program_2022 = accounts.get(12)?;
        let memo_program = accounts.get(13)?;
        let vault_0_mint = accounts.get(14)?;
        let vault_1_mint = accounts.get(15)?;

        Some(DecreaseLiquidityV2InstructionAccounts {
            nft_owner: nft_owner.pubkey,
            nft_account: nft_account.pubkey,
            personal_position: personal_position.pubkey,
            pool_state: pool_state.pubkey,
            protocol_position: protocol_position.pubkey,
            token_vault_0: token_vault_0.pubkey,
            token_vault_1: token_vault_1.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            recipient_token_account_0: recipient_token_account_0.pubkey,
            recipient_token_account_1: recipient_token_account_1.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
            memo_program: memo_program.pubkey,
            vault_0_mint: vault_0_mint.pubkey,
            vault_1_mint: vault_1_mint.pubkey,
        })
    }
}
