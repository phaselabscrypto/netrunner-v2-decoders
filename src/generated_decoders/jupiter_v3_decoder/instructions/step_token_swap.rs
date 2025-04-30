use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x376411f3f2b52ba5")]
pub struct StepTokenSwap {
    pub in_amount: Option<u64>,
    pub minimum_out_amount: u64,
    pub platform_fee_bps: u8,
}

pub struct StepTokenSwapInstructionAccounts {
    pub token_swap_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub swap: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source: solana_sdk::pubkey::Pubkey,
    pub swap_source: solana_sdk::pubkey::Pubkey,
    pub swap_destination: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub pool_mint: solana_sdk::pubkey::Pubkey,
    pub pool_fee: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StepTokenSwap {
    type ArrangedAccounts = StepTokenSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_swap_program = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let swap = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let user_transfer_authority = accounts.get(4)?;
        let source = accounts.get(5)?;
        let swap_source = accounts.get(6)?;
        let swap_destination = accounts.get(7)?;
        let destination = accounts.get(8)?;
        let pool_mint = accounts.get(9)?;
        let pool_fee = accounts.get(10)?;

        Some(StepTokenSwapInstructionAccounts {
            token_swap_program: token_swap_program.pubkey,
            token_program: token_program.pubkey,
            swap: swap.pubkey,
            authority: authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source: source.pubkey,
            swap_source: swap_source.pubkey,
            swap_destination: swap_destination.pubkey,
            destination: destination.pubkey,
            pool_mint: pool_mint.pubkey,
            pool_fee: pool_fee.pubkey,
        })
    }
}
