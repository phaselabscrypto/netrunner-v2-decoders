use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateAndDelegateStakeAccountParams {
    pub stake_account_index: u64,
    pub stake_amount_lamports: u64,
}
