use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::TitanSwapDecoder;
pub mod atlas;

pub enum TitanSwapAccount {
    Atlas(Box<atlas::Atlas>),
}

impl<'a> AccountDecoder<'a> for TitanSwapDecoder {
    type AccountType = TitanSwapAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = atlas::Atlas::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TitanSwapAccount::Atlas(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
