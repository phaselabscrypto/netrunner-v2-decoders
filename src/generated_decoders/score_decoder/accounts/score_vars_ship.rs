
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x1bda89996a86da8f")] 
pub struct ScoreVarsShip { 
        pub ship_mint: solana_sdk::pubkey::Pubkey, 
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