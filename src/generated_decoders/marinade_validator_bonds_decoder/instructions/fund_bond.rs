use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3a2cd4af1e11443e")]
pub struct FundBond {}

pub struct FundBondInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub bonds_withdrawer_authority: solana_sdk::pubkey::Pubkey,
    pub stake_account: solana_sdk::pubkey::Pubkey,
    pub stake_authority: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub stake_history: solana_sdk::pubkey::Pubkey,
    pub stake_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FundBond {
    type ArrangedAccounts = FundBondInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let bonds_withdrawer_authority = accounts.get(2)?;
        let stake_account = accounts.get(3)?;
        let stake_authority = accounts.get(4)?;
        let clock = accounts.get(5)?;
        let stake_history = accounts.get(6)?;
        let stake_program = accounts.get(7)?;
        let event_authority = accounts.get(8)?;
        let program = accounts.get(9)?;

        Some(FundBondInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            bonds_withdrawer_authority: bonds_withdrawer_authority.pubkey,
            stake_account: stake_account.pubkey,
            stake_authority: stake_authority.pubkey,
            clock: clock.pubkey,
            stake_history: stake_history.pubkey,
            stake_program: stake_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
