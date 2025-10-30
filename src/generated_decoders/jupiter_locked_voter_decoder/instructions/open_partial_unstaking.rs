use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc989cfaf4f5fdc1b")]
pub struct OpenPartialUnstaking {
    pub amount: u64,
    pub memo: String,
}

pub struct OpenPartialUnstakingInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub partial_unstake: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPartialUnstaking {
    type ArrangedAccounts = OpenPartialUnstakingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, partial_unstake, owner, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(OpenPartialUnstakingInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            partial_unstake: partial_unstake.pubkey,
            owner: owner.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
