use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5c2cd2bbac0640b7")]
pub struct ThawNft {}

pub struct ThawNftInstructionAccounts {
    pub freeze_pda: solana_sdk::pubkey::Pubkey,
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ThawNft {
    type ArrangedAccounts = ThawNftInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let freeze_pda = accounts.get(0)?;
        let candy_machine = accounts.get(1)?;
        let token_account = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let mint = accounts.get(4)?;
        let edition = accounts.get(5)?;
        let payer = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let token_metadata_program = accounts.get(8)?;
        let system_program = accounts.get(9)?;

        Some(ThawNftInstructionAccounts {
            freeze_pda: freeze_pda.pubkey,
            candy_machine: candy_machine.pubkey,
            token_account: token_account.pubkey,
            owner: owner.pubkey,
            mint: mint.pubkey,
            edition: edition.pubkey,
            payer: payer.pubkey,
            token_program: token_program.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
