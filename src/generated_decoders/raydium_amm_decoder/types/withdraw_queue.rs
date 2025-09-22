
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WithdrawQueue {
    pub owner: [u64; 4],
    pub head: u64,
    pub count: u64,
    #[serde(with = "BigArray")]
    pub buf: [WithdrawDestToken; 64],
}
