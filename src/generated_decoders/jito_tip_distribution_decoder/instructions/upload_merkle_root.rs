use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x46036e1dc7becdb0")]
pub struct UploadMerkleRoot {
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub max_num_nodes: u64,
}

pub struct UploadMerkleRootInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
    pub merkle_root_upload_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UploadMerkleRoot {
    type ArrangedAccounts = UploadMerkleRootInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let tip_distribution_account = accounts.get(1)?;
        let merkle_root_upload_authority = accounts.get(2)?;

        Some(UploadMerkleRootInstructionAccounts {
            config: config.pubkey,
            tip_distribution_account: tip_distribution_account.pubkey,
            merkle_root_upload_authority: merkle_root_upload_authority.pubkey,
        })
    }
}
