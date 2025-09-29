

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc198255408300f5a")]
pub struct ProcessRegisterShip{
    pub scorevars_bump: u8,
    pub scorevars_ship_bump: u8,
    pub reward_rate_per_second: u64,
    pub fuel_max_reserve: u32,
    pub food_max_reserve: u32,
    pub arms_max_reserve: u32,
    pub toolkit_max_reserve: u32,
    pub milliseconds_to_burn_one_fuel: u32,
    pub milliseconds_to_burn_one_food: u32,
    pub milliseconds_to_burn_one_arms: u32,
    pub milliseconds_to_burn_one_toolkit: u32,
}

pub struct ProcessRegisterShipInstructionAccounts {
    pub update_authority_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_account: solana_sdk::pubkey::Pubkey,
    pub score_vars_ship_account: solana_sdk::pubkey::Pubkey,
    pub ship_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ProcessRegisterShip {
    type ArrangedAccounts = ProcessRegisterShipInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            update_authority_account,
            score_vars_account,
            score_vars_ship_account,
            ship_mint,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(ProcessRegisterShipInstructionAccounts {
            update_authority_account: update_authority_account.pubkey,
            score_vars_account: score_vars_account.pubkey,
            score_vars_ship_account: score_vars_ship_account.pubkey,
            ship_mint: ship_mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}