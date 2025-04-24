use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x61a7906b75be8024")]
pub struct OrderUnstake {
    pub msol_amount: u64,
}

pub struct OrderUnstakeInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub msol_mint: solana_sdk::pubkey::Pubkey,
    pub burn_msol_from: solana_sdk::pubkey::Pubkey,
    pub burn_msol_authority: solana_sdk::pubkey::Pubkey,
    pub new_ticket_account: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OrderUnstake {
    type ArrangedAccounts = OrderUnstakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let state = accounts.get(0)?;
        let msol_mint = accounts.get(1)?;
        let burn_msol_from = accounts.get(2)?;
        let burn_msol_authority = accounts.get(3)?;
        let new_ticket_account = accounts.get(4)?;
        let clock = accounts.get(5)?;
        let rent = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(OrderUnstakeInstructionAccounts {
            state: state.pubkey,
            msol_mint: msol_mint.pubkey,
            burn_msol_from: burn_msol_from.pubkey,
            burn_msol_authority: burn_msol_authority.pubkey,
            new_ticket_account: new_ticket_account.pubkey,
            clock: clock.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
