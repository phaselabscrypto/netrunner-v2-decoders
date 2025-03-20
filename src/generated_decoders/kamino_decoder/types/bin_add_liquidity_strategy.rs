

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum BinAddLiquidityStrategy {
    Uniform
                {
                    current_bin_index: i32,
                    lower_bin_index: i32,
                    upper_bin_index: i32,
                    amount_x_to_deposit: u64,
                    amount_y_to_deposit: u64,
                    x_current_bin: u64,
                    y_current_bin: u64,
                }
    ,
    CurrentTick
                (
                    i32,
                )
    ,
}


