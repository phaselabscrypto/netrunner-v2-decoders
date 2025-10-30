use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd8b68f0bdc2656b9")]
pub struct NewEscrow {}

pub struct NewEscrowInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for NewEscrow {
    type ArrangedAccounts = NewEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, escrow_owner, payer, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(NewEscrowInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            escrow_owner: escrow_owner.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
