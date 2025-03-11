

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xd33906a70fdb23fb")]
pub struct MintNft{
    pub creator_bump: u8,
}

pub struct MintNftInstructionAccounts {
    pub candy_machine: solana_sdk::pubkey::Pubkey,
    pub candy_machine_creator: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub token_metadata_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub clock: solana_sdk::pubkey::Pubkey,
    pub recent_blockhashes: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintNft {
    type ArrangedAccounts = MintNftInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let candy_machine = accounts.get(0)?;
        let candy_machine_creator = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let wallet = accounts.get(3)?;
        let metadata = accounts.get(4)?;
        let mint = accounts.get(5)?;
        let mint_authority = accounts.get(6)?;
        let update_authority = accounts.get(7)?;
        let master_edition = accounts.get(8)?;
        let token_metadata_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let rent = accounts.get(12)?;
        let clock = accounts.get(13)?;
        let recent_blockhashes = accounts.get(14)?;
        let instruction_sysvar_account = accounts.get(15)?;

        Some(MintNftInstructionAccounts {
            candy_machine: candy_machine.pubkey,
            candy_machine_creator: candy_machine_creator.pubkey,
            payer: payer.pubkey,
            wallet: wallet.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            update_authority: update_authority.pubkey,
            master_edition: master_edition.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            clock: clock.pubkey,
            recent_blockhashes: recent_blockhashes.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}