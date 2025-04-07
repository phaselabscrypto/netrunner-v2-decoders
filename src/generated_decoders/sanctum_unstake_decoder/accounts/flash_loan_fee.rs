
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xd371d38abf6c40a0")] 
pub struct FlashLoanFee { 
        pub fee_ratio: Rational, 
}