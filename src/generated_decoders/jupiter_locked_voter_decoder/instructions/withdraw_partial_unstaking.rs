use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc9ca897c0203f557")]
pub struct WithdrawPartialUnstaking {}

pub struct WithdrawPartialUnstakingInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub partial_unstake: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub escrow_tokens: solana_sdk::pubkey::Pubkey,
    pub destination_tokens: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawPartialUnstaking {
    type ArrangedAccounts = WithdrawPartialUnstakingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, partial_unstake, owner, escrow_tokens, destination_tokens, payer, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawPartialUnstakingInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            partial_unstake: partial_unstake.pubkey,
            owner: owner.pubkey,
            escrow_tokens: escrow_tokens.pubkey,
            destination_tokens: destination_tokens.pubkey,
            payer: payer.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
