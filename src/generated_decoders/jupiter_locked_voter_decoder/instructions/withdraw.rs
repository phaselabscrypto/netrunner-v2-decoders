use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {}

pub struct WithdrawInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
    pub escrow_tokens: solana_sdk::pubkey::Pubkey,
    pub destination_tokens: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, escrow_owner, escrow_tokens, destination_tokens, payer, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            escrow_owner: escrow_owner.pubkey,
            escrow_tokens: escrow_tokens.pubkey,
            destination_tokens: destination_tokens.pubkey,
            payer: payer.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
