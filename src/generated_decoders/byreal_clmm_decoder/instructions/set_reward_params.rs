use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7034a74b20c9d389")]
pub struct SetRewardParams {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
    pub open_time: u64,
    pub end_time: u64,
}

pub struct SetRewardParamsInstructionAccounts {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub operation_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program_2022: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardParams {
    type ArrangedAccounts = SetRewardParamsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let authority = accounts.get(0)?;
        let admin_group = accounts.get(1)?;
        let amm_config = accounts.get(2)?;
        let pool_state = accounts.get(3)?;
        let operation_state = accounts.get(4)?;
        let token_program = accounts.get(5)?;
        let token_program_2022 = accounts.get(6)?;

        Some(SetRewardParamsInstructionAccounts {
            authority: authority.pubkey,
            admin_group: admin_group.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            operation_state: operation_state.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
        })
    }
}
