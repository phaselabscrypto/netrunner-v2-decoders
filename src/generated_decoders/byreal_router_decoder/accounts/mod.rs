use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::AmmRoutingDecoder;
pub mod pool_config;

pub enum AmmRoutingAccount {
    PoolConfig(pool_config::PoolConfig),
}

impl<'a> AccountDecoder<'a> for AmmRoutingDecoder {
    type AccountType = AmmRoutingAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = pool_config::PoolConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AmmRoutingAccount::PoolConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
