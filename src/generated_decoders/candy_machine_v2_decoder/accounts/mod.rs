 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::CandyMachineDecoder; 
pub mod candy_machine; 
pub mod collection_pda; 
pub mod freeze_pda; 

pub enum CandyMachineAccount { 
        CandyMachine(candy_machine::CandyMachine), 
        CollectionPda(collection_pda::CollectionPda), 
        FreezePda(freeze_pda::FreezePda), 
}


impl<'a> AccountDecoder<'a> for CandyMachineDecoder { 
    type AccountType = CandyMachineAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = candy_machine::CandyMachine::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CandyMachineAccount::CandyMachine(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = collection_pda::CollectionPda::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CandyMachineAccount::CollectionPda(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = freeze_pda::FreezePda::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: CandyMachineAccount::FreezePda(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}