use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::NftCandyMachineDecoder;
pub mod candy_machine;
pub mod config;

pub enum NftCandyMachineAccount {
    CandyMachine(candy_machine::CandyMachine),
    Config(config::Config),
}

impl<'a> AccountDecoder<'a> for NftCandyMachineDecoder {
    type AccountType = NftCandyMachineAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            candy_machine::CandyMachine::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: NftCandyMachineAccount::CandyMachine(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = config::Config::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: NftCandyMachineAccount::Config(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
