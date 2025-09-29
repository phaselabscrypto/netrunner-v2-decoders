
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x43e55428bc25097b")] 
pub struct ShipStaking { 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub faction_id: u8, 
        pub ship_mint: solana_sdk::pubkey::Pubkey, 
        pub ship_quantity_in_escrow: u64, 
        pub fuel_quantity_in_escrow: u64, 
        pub food_quantity_in_escrow: u64, 
        pub arms_quantity_in_escrow: u64, 
        pub fuel_current_capacity: u64, 
        pub food_current_capacity: u64, 
        pub arms_current_capacity: u64, 
        pub health_current_capacity: u64, 
        pub staked_at_timestamp: i64, 
        pub fueled_at_timestamp: i64, 
        pub fed_at_timestamp: i64, 
        pub armed_at_timestamp: i64, 
        pub repaired_at_timestamp: i64, 
        pub current_capacity_timestamp: i64, 
        pub total_time_staked: u64, 
        pub staked_time_paid: u64, 
        pub pending_rewards: u64, 
        pub total_rewards_paid: u64, 
}