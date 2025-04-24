use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd3e7c245410b7b5d")]
pub struct CreatePermissionedEscrowWithAuthority {
    pub max_cap: u64,
}

pub struct CreatePermissionedEscrowWithAuthorityInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePermissionedEscrowWithAuthority {
    type ArrangedAccounts = CreatePermissionedEscrowWithAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let escrow = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let payer = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let event_authority = accounts.get(6)?;
        let program = accounts.get(7)?;

        Some(CreatePermissionedEscrowWithAuthorityInstructionAccounts {
            vault: vault.pubkey,
            pool: pool.pubkey,
            escrow: escrow.pubkey,
            owner: owner.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
