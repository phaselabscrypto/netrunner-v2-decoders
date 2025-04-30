use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x273a2680643ebff9")]
pub struct WhirlpoolSwapExactOutput {
    pub out_amount: u64,
    pub maximum_in_amount: u64,
    pub a_to_b: bool,
    pub platform_fee_bps: u8,
}

pub struct WhirlpoolSwapExactOutputInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array0: solana_sdk::pubkey::Pubkey,
    pub tick_array1: solana_sdk::pubkey::Pubkey,
    pub tick_array2: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WhirlpoolSwapExactOutput {
    type ArrangedAccounts = WhirlpoolSwapExactOutputInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let token_authority = accounts.get(2)?;
        let whirlpool = accounts.get(3)?;
        let token_owner_account_a = accounts.get(4)?;
        let token_vault_a = accounts.get(5)?;
        let token_owner_account_b = accounts.get(6)?;
        let token_vault_b = accounts.get(7)?;
        let tick_array0 = accounts.get(8)?;
        let tick_array1 = accounts.get(9)?;
        let tick_array2 = accounts.get(10)?;
        let oracle = accounts.get(11)?;

        Some(WhirlpoolSwapExactOutputInstructionAccounts {
            swap_program: swap_program.pubkey,
            token_program: token_program.pubkey,
            token_authority: token_authority.pubkey,
            whirlpool: whirlpool.pubkey,
            token_owner_account_a: token_owner_account_a.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_owner_account_b: token_owner_account_b.pubkey,
            token_vault_b: token_vault_b.pubkey,
            tick_array0: tick_array0.pubkey,
            tick_array1: tick_array1.pubkey,
            tick_array2: tick_array2.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
