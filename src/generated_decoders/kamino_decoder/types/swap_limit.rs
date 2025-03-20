

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum SwapLimit {
    Bps
                (
                    u64,
                )
    ,
    Absolute
                {
                    src_amount_to_swap: u64,
                    dst_amount_to_vault: u64,
                    a_to_b: bool,
                }
    ,
}


