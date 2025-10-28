use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::JupiterMerkleDistributorDecoder;
pub mod claim_status;
pub mod merkle_distributor;

pub enum JupiterMerkleDistributorAccount {
    ClaimStatus(claim_status::ClaimStatus),
    MerkleDistributor(merkle_distributor::MerkleDistributor),
}

impl<'a> AccountDecoder<'a> for JupiterMerkleDistributorDecoder {
    type AccountType = JupiterMerkleDistributorAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            claim_status::ClaimStatus::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterMerkleDistributorAccount::ClaimStatus(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            merkle_distributor::MerkleDistributor::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterMerkleDistributorAccount::MerkleDistributor(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
