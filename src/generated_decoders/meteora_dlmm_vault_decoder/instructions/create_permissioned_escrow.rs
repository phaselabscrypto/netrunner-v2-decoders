use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3ca62455608984b8")]
pub struct CreatePermissionedEscrow {
    pub max_cap: u64,
    pub proof: Vec<[u8; 32]>,
}

pub struct CreatePermissionedEscrowInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub merkle_root_config: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub escrow_fee_receiver: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreatePermissionedEscrow {
    type ArrangedAccounts = CreatePermissionedEscrowInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let pool = accounts.get(1)?;
        let escrow = accounts.get(2)?;
        let owner = accounts.get(3)?;
        let merkle_root_config = accounts.get(4)?;
        let payer = accounts.get(5)?;
        let escrow_fee_receiver = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let event_authority = accounts.get(8)?;
        let program = accounts.get(9)?;

        Some(CreatePermissionedEscrowInstructionAccounts {
            vault: vault.pubkey,
            pool: pool.pubkey,
            escrow: escrow.pubkey,
            owner: owner.pubkey,
            merkle_root_config: merkle_root_config.pubkey,
            payer: payer.pubkey,
            escrow_fee_receiver: escrow_fee_receiver.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
