use carbon_core::account::AccountDecoder;

use super::JupiterDecoder;

pub enum JupiterAccount {}

impl<'a> AccountDecoder<'a> for JupiterDecoder {
    type AccountType = JupiterAccount;
    fn decode_account(
        &self,
        _account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
