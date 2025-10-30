use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::LockedVoterDecoder;
pub mod escrow;
pub mod locker;
pub mod partial_unstaking;

pub enum LockedVoterAccount {
    Locker(locker::Locker),
    Escrow(escrow::Escrow),
    PartialUnstaking(partial_unstaking::PartialUnstaking),
}

impl<'a> AccountDecoder<'a> for LockedVoterDecoder {
    type AccountType = LockedVoterAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = locker::Locker::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LockedVoterAccount::Locker(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = escrow::Escrow::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LockedVoterAccount::Escrow(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            partial_unstaking::PartialUnstaking::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LockedVoterAccount::PartialUnstaking(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
