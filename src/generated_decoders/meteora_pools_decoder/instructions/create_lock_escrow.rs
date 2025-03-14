

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x3657a51345e3dae0")]
pub struct CreateLockEscrow{
}

pub struct CreateLockEscrowInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub lock_escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateLockEscrow {
    type ArrangedAccounts = CreateLockEscrowInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let pool = accounts.get(0)?;
        let lock_escrow = accounts.get(1)?;
        let owner = accounts.get(2)?;
        let lp_mint = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let system_program = accounts.get(5)?;

        Some(CreateLockEscrowInstructionAccounts {
            pool: pool.pubkey,
            lock_escrow: lock_escrow.pubkey,
            owner: owner.pubkey,
            lp_mint: lp_mint.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}