use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xe3aaa4da7f1023df")]
pub struct CustomOracle {
    pub price: u64,
    pub expo: i32,
    pub conf: u64,
    pub ema: u64,
    pub publish_time: i64,
    pub ext_oracle_account: solana_sdk::pubkey::Pubkey,
}
