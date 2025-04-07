 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::UnstakeDecoder; 
pub mod fee; 
pub mod flash_loan_fee; 
pub mod pool; 
pub mod protocol_fee; 
pub mod stake_account_record; 

pub enum UnstakeAccount { 
        Fee(fee::Fee), 
        FlashLoanFee(flash_loan_fee::FlashLoanFee), 
        Pool(pool::Pool), 
        ProtocolFee(protocol_fee::ProtocolFee), 
        StakeAccountRecord(stake_account_record::StakeAccountRecord), 
}


impl<'a> AccountDecoder<'a> for UnstakeDecoder { 
    type AccountType = UnstakeAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = fee::Fee::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: UnstakeAccount::Fee(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = flash_loan_fee::FlashLoanFee::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: UnstakeAccount::FlashLoanFee(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = pool::Pool::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: UnstakeAccount::Pool(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = protocol_fee::ProtocolFee::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: UnstakeAccount::ProtocolFee(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = stake_account_record::StakeAccountRecord::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: UnstakeAccount::StakeAccountRecord(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}