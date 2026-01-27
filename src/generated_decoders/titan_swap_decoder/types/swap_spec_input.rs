use carbon_core::borsh;

#[derive(
    Debug,
    Clone,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct SwapSpecInput {
    pub venue: super::Venue,
    pub from: u8,
    pub to: u8,
    pub weight_bps: u16,
    pub minimum_amount_out: u64,
    pub n_accounts: u8,
}
