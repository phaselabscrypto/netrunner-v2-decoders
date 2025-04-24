use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::AmmProgramDecoder;
pub mod asset_deposit_receipt;
pub mod nft_deposit_receipt;
pub mod pool;

pub enum AmmProgramAccount {
    AssetDepositReceipt(asset_deposit_receipt::AssetDepositReceipt),
    NftDepositReceipt(nft_deposit_receipt::NftDepositReceipt),
    Pool(pool::Pool),
}

impl<'a> AccountDecoder<'a> for AmmProgramDecoder {
    type AccountType = AmmProgramAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            asset_deposit_receipt::AssetDepositReceipt::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AmmProgramAccount::AssetDepositReceipt(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            nft_deposit_receipt::NftDepositReceipt::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AmmProgramAccount::NftDepositReceipt(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: AmmProgramAccount::Pool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
