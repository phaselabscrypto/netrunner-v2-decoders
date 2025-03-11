 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::CandyMachineCoreDecoder; 
pub mod candy_machine; 

pub enum CandyMachineCoreAccount { 
        CandyMachine(candy_machine::CandyMachine), 
}


impl<'a> AccountDecoder<'a> for CandyMachineCoreDecoder { 
    type AccountType = CandyMachineCoreAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = candy_machine::CandyMachine::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CandyMachineCoreAccount::CandyMachine(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}