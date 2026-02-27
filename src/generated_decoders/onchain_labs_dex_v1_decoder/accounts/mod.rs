use carbon_core::account::AccountDecoder;

use super::OnchainLabsDexV1Decoder;

#[derive(Debug, Clone, PartialEq)]
pub enum OnchainLabsDexV1Account {}

impl<'a> AccountDecoder<'a> for OnchainLabsDexV1Decoder {
    type AccountType = OnchainLabsDexV1Account;

    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        let _data = account.data.as_slice();

        None
    }
}
