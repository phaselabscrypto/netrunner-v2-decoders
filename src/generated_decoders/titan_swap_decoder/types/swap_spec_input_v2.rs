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
pub struct SwapSpecInputV2 {
    pub venue: super::Venue,
    pub from: u8,
    pub to: u8,
    pub weight_nanos: u32,
    pub n_accounts: u8,
}
