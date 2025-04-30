use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf43ffac0322cacfa")]
pub struct InitializeTokenLedger {}

pub struct InitializeTokenLedgerInstructionAccounts {
    pub token_ledger: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTokenLedger {
    type ArrangedAccounts = InitializeTokenLedgerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let token_ledger = accounts.get(0)?;
        let payer = accounts.get(1)?;
        let system_program = accounts.get(2)?;
        let rent = accounts.get(3)?;

        Some(InitializeTokenLedgerInstructionAccounts {
            token_ledger: token_ledger.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
