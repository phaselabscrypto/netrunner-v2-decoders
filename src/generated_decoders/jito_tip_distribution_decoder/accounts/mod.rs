use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::JitoTipDistributionDecoder;
pub mod claim_status;
pub mod config;
pub mod tip_distribution_account;

pub enum JitoTipDistributionAccount {
    Config(config::Config),
    TipDistributionAccount(tip_distribution_account::TipDistributionAccount),
    ClaimStatus(claim_status::ClaimStatus),
}

impl<'a> AccountDecoder<'a> for JitoTipDistributionDecoder {
    type AccountType = JitoTipDistributionAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = config::Config::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JitoTipDistributionAccount::Config(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            tip_distribution_account::TipDistributionAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JitoTipDistributionAccount::TipDistributionAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            claim_status::ClaimStatus::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JitoTipDistributionAccount::ClaimStatus(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
