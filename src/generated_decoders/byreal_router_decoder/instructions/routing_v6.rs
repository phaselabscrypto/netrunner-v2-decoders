use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ce70cdf2bb96094")]
pub struct RoutingV6 {
    pub amount_in: u64,
    pub amount_out_minimum: u64,
    pub swap_account_counts: Vec<u8>,
}

pub struct RoutingV6InstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program_2022: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RoutingV6 {
    type ArrangedAccounts = RoutingV6InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let input_token_account = accounts.get(1)?;
        let input_token_mint = accounts.get(2)?;
        let token_program = accounts.get(3)?;
        let token_program_2022 = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let associated_token_program = accounts.get(6)?;
        let memo_program = accounts.get(7)?;

        Some(RoutingV6InstructionAccounts {
            payer: payer.pubkey,
            input_token_account: input_token_account.pubkey,
            input_token_mint: input_token_mint.pubkey,
            token_program: token_program.pubkey,
            token_program_2022: token_program_2022.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
