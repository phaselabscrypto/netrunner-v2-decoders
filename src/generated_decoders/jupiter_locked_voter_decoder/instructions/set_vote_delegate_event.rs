use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da5a09df179223608")]
pub struct SetVoteDelegateEvent {
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
    pub old_delegate: solana_sdk::pubkey::Pubkey,
    pub new_delegate: solana_sdk::pubkey::Pubkey,
}
