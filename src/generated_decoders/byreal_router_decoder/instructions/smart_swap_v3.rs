use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1ef163dcdeda1233")]
pub struct SmartSwapV3 {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
}

pub struct SmartSwapV3InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub dex_program: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub output_vault: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub user_input_token_account: solana_sdk::pubkey::Pubkey,
    pub user_output_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program_2022: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SmartSwapV3 {
    type ArrangedAccounts = SmartSwapV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let dex_program = accounts.get(1)?;
        let pool_state = accounts.get(2)?;
        let input_vault = accounts.get(3)?;
        let output_vault = accounts.get(4)?;
        let observation_state = accounts.get(5)?;
        let input_mint = accounts.get(6)?;
        let output_mint = accounts.get(7)?;
        let user_input_token_account = accounts.get(8)?;
        let user_output_token_account = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let token_program_2022 = accounts.get(11)?;
        let system_program = accounts.get(12)?;
        let associated_token_program = accounts.get(13)?;
        let memo_program = accounts.get(14)?;

        Some(SmartSwapV3InstructionAccounts {
            payer: payer.pubkey,
            dex_program: dex_program.pubkey,
            pool_state: pool_state.pubkey,
            input_vault: input_vault.pubkey,
            output_vault: output_vault.pubkey,
            observation_state: observation_state.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            user_input_token_account: user_input_token_account.pubkey,
            user_output_token_account: user_output_token_account.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
