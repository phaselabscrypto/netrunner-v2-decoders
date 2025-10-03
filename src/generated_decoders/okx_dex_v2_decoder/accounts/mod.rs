use carbon_core::account::AccountDecoder;

use super::OkxDexV2Decoder;

pub enum OkxDexV2Account {}

impl<'a> AccountDecoder<'a> for OkxDexV2Decoder {
    type AccountType = OkxDexV2Account;
    fn decode_account(
        &self,
        _account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        None
    }
}
