use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd28ed631d569c652")]
pub struct CancelTriggerOrderLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub price: u64,
    pub price_exponent: i32,
    pub size_amount: u64,
    pub receive_custody_uid: u8,
    pub is_stop_loss: bool,
}
