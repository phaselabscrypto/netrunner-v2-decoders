use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xea5e55e1a766a920")]
pub struct MintBond {}

pub struct MintBondInstructionAccounts {
    pub config: solana_sdk::pubkey::Pubkey,
    pub bond: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub validator_identity: solana_sdk::pubkey::Pubkey,
    pub validator_identity_token_account: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub rent_payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintBond {
    type ArrangedAccounts = MintBondInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let config = accounts.get(0)?;
        let bond = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let validator_identity = accounts.get(3)?;
        let validator_identity_token_account = accounts.get(4)?;
        let vote_account = accounts.get(5)?;
        let metadata = accounts.get(6)?;
        let rent_payer = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let metadata_program = accounts.get(11)?;
        let rent = accounts.get(12)?;
        let event_authority = accounts.get(13)?;
        let program = accounts.get(14)?;

        Some(MintBondInstructionAccounts {
            config: config.pubkey,
            bond: bond.pubkey,
            mint: mint.pubkey,
            validator_identity: validator_identity.pubkey,
            validator_identity_token_account: validator_identity_token_account.pubkey,
            vote_account: vote_account.pubkey,
            metadata: metadata.pubkey,
            rent_payer: rent_payer.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
