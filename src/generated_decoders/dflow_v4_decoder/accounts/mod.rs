use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::SwapOrchestratorDecoder;
pub mod order;

pub enum SwapOrchestratorAccount {
    Order(order::Order),
}

impl<'a> AccountDecoder<'a> for SwapOrchestratorDecoder {
    type AccountType = SwapOrchestratorAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = order::Order::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: SwapOrchestratorAccount::Order(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
