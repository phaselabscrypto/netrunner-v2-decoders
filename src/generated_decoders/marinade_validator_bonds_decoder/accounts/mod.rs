use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::ValidatorBondsDecoder;
pub mod bond;
pub mod config;
pub mod settlement;
pub mod settlement_claim;
pub mod settlement_claims;
pub mod withdraw_request;

pub enum ValidatorBondsAccount {
    SettlementClaim(settlement_claim::SettlementClaim),
    Bond(bond::Bond),
    Config(config::Config),
    SettlementClaims(settlement_claims::SettlementClaims),
    Settlement(settlement::Settlement),
    WithdrawRequest(withdraw_request::WithdrawRequest),
}

impl<'a> AccountDecoder<'a> for ValidatorBondsDecoder {
    type AccountType = ValidatorBondsAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            settlement_claim::SettlementClaim::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ValidatorBondsAccount::SettlementClaim(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = bond::Bond::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ValidatorBondsAccount::Bond(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = config::Config::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ValidatorBondsAccount::Config(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            settlement_claims::SettlementClaims::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ValidatorBondsAccount::SettlementClaims(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = settlement::Settlement::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ValidatorBondsAccount::Settlement(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            withdraw_request::WithdrawRequest::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: ValidatorBondsAccount::WithdrawRequest(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
