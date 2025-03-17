

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x4dffae527d1dc92e")]
pub struct OpenPositionWithToken22Nft{
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
    pub liquidity: u128,
    pub amount0_max: u64,
    pub amount1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
}

pub struct OpenPositionWithToken22NftInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub position_nft_owner: solana_sdk::pubkey::Pubkey,
    pub position_nft_mint: solana_sdk::pubkey::Pubkey,
    pub position_nft_account: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub protocol_position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub personal_position: solana_sdk::pubkey::Pubkey,
    pub token_account0: solana_sdk::pubkey::Pubkey,
    pub token_account1: solana_sdk::pubkey::Pubkey,
    pub token_vault0: solana_sdk::pubkey::Pubkey,
    pub token_vault1: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub vault0_mint: solana_sdk::pubkey::Pubkey,
    pub vault1_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPositionWithToken22Nft {
    type ArrangedAccounts = OpenPositionWithToken22NftInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let position_nft_owner = accounts.get(1)?;
        let position_nft_mint = accounts.get(2)?;
        let position_nft_account = accounts.get(3)?;
        let pool_state = accounts.get(4)?;
        let protocol_position = accounts.get(5)?;
        let tick_array_lower = accounts.get(6)?;
        let tick_array_upper = accounts.get(7)?;
        let personal_position = accounts.get(8)?;
        let token_account0 = accounts.get(9)?;
        let token_account1 = accounts.get(10)?;
        let token_vault0 = accounts.get(11)?;
        let token_vault1 = accounts.get(12)?;
        let rent = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let associated_token_program = accounts.get(16)?;
        let token_program2022 = accounts.get(17)?;
        let vault0_mint = accounts.get(18)?;
        let vault1_mint = accounts.get(19)?;

        Some(OpenPositionWithToken22NftInstructionAccounts {
            payer: payer.pubkey,
            position_nft_owner: position_nft_owner.pubkey,
            position_nft_mint: position_nft_mint.pubkey,
            position_nft_account: position_nft_account.pubkey,
            pool_state: pool_state.pubkey,
            protocol_position: protocol_position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            personal_position: personal_position.pubkey,
            token_account0: token_account0.pubkey,
            token_account1: token_account1.pubkey,
            token_vault0: token_vault0.pubkey,
            token_vault1: token_vault1.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            vault0_mint: vault0_mint.pubkey,
            vault1_mint: vault1_mint.pubkey,
        })
    }
}