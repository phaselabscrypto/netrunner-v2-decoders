
 
use carbon_core::{borsh, CarbonDeserialize};
use serde::{Serialize, Deserialize};

#[derive(CarbonDeserialize, Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)] 
 

#[carbon(discriminator = "0x979d32738248b324")] 
pub struct Fees { 
        pub min_separate_numerator: u64, 
        pub min_separate_denominator: u64, 
        pub trade_fee_numerator: u64, 
        pub trade_fee_denominator: u64, 
        pub pnl_numerator: u64, 
        pub pnl_denominator: u64, 
        pub swap_fee_numerator: u64, 
        pub swap_fee_denominator: u64, 
}