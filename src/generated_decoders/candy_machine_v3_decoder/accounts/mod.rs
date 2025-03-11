 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::CandyGuardDecoder; 
pub mod freeze_escrow; 
pub mod candy_guard; 

pub enum CandyGuardAccount { 
        FreezeEscrow(freeze_escrow::FreezeEscrow), 
        CandyGuard(candy_guard::CandyGuard), 
}


impl<'a> AccountDecoder<'a> for CandyGuardDecoder { 
    type AccountType = CandyGuardAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = freeze_escrow::FreezeEscrow::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CandyGuardAccount::FreezeEscrow(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = candy_guard::CandyGuard::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CandyGuardAccount::CandyGuard(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}