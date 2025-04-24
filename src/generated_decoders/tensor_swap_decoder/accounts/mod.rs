use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::TensorswapDecoder;
pub mod margin_account;
pub mod nft_authority;
pub mod nft_deposit_receipt;
pub mod pool;
pub mod single_listing;
pub mod sol_escrow;
pub mod t_swap;

pub enum TensorswapAccount {
    TSwap(t_swap::TSwap),
    Pool(pool::Pool),
    MarginAccount(margin_account::MarginAccount),
    SingleListing(single_listing::SingleListing),
    NftDepositReceipt(nft_deposit_receipt::NftDepositReceipt),
    NftAuthority(nft_authority::NftAuthority),
    SolEscrow(sol_escrow::SolEscrow),
}

impl<'a> AccountDecoder<'a> for TensorswapDecoder {
    type AccountType = TensorswapAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = t_swap::TSwap::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TensorswapAccount::TSwap(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TensorswapAccount::Pool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            margin_account::MarginAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TensorswapAccount::MarginAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            single_listing::SingleListing::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TensorswapAccount::SingleListing(decoded_account),
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
                data: TensorswapAccount::NftDepositReceipt(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            nft_authority::NftAuthority::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TensorswapAccount::NftAuthority(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = sol_escrow::SolEscrow::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: TensorswapAccount::SolEscrow(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
