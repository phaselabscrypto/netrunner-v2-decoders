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
pub struct SwapDetails {
    pub index: u8,
    pub input_amount: u64,
    pub output_amount: u64,
}
