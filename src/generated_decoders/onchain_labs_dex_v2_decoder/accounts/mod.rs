use carbon_core::account::AccountDecoder;

use super::OnchainLabsDexV2Decoder;

pub mod token_ledger;

#[derive(Debug, Clone, PartialEq)]
pub enum OnchainLabsDexV2Account {
    TokenLedger(Box<token_ledger::TokenLedger>),
}

impl<'a> AccountDecoder<'a> for OnchainLabsDexV2Decoder {
    type AccountType = OnchainLabsDexV2Account;

    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        let data = account.data.as_slice();

        if let Some(decoded) = token_ledger::TokenLedger::decode(data) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OnchainLabsDexV2Account::TokenLedger(Box::new(decoded)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
