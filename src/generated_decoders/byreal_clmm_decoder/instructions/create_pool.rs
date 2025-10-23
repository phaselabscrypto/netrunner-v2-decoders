use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe992d18ecf6840bc")]
pub struct CreatePool {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

pub struct CreatePoolInstructionAccounts {
    pub pool_creator: solana_sdk::pubkey::Pubkey,
    pub pool_manager: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub offchain_reward_config: solana_sdk::pubkey::Pubkey,
    pub token_mint_0: solana_sdk::pubkey::Pubkey,
    pub token_mint_1: solana_sdk::pubkey::Pubkey,
    pub token_vault_0: solana_sdk::pubkey::Pubkey,
    pub token_vault_1: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
    pub tick_array_bitmap: solana_sdk::pubkey::Pubkey,
    pub token_program_0: solana_sdk::pubkey::Pubkey,
    pub token_program_1: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePool {
    type ArrangedAccounts = CreatePoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let pool_creator = accounts.get(0)?;
        let pool_manager = accounts.get(1)?;
        let admin_group = accounts.get(2)?;
        let amm_config = accounts.get(3)?;
        let pool_state = accounts.get(4)?;
        let offchain_reward_config = accounts.get(5)?;
        let token_mint_0 = accounts.get(6)?;
        let token_mint_1 = accounts.get(7)?;
        let token_vault_0 = accounts.get(8)?;
        let token_vault_1 = accounts.get(9)?;
        let observation_state = accounts.get(10)?;
        let tick_array_bitmap = accounts.get(11)?;
        let token_program_0 = accounts.get(12)?;
        let token_program_1 = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let rent = accounts.get(15)?;

        Some(CreatePoolInstructionAccounts {
            pool_creator: pool_creator.pubkey,
            pool_manager: pool_manager.pubkey,
            admin_group: admin_group.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            offchain_reward_config: offchain_reward_config.pubkey,
            token_mint_0: token_mint_0.pubkey,
            token_mint_1: token_mint_1.pubkey,
            token_vault_0: token_vault_0.pubkey,
            token_vault_1: token_vault_1.pubkey,
            observation_state: observation_state.pubkey,
            tick_array_bitmap: tick_array_bitmap.pubkey,
            token_program_0: token_program_0.pubkey,
            token_program_1: token_program_1.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
