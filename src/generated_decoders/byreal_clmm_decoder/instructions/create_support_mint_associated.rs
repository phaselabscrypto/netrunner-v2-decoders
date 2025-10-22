use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11fb415c88f20ea9")]
pub struct CreateSupportMintAssociated {}

pub struct CreateSupportMintAssociatedInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub admin_group: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub support_mint_associated: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateSupportMintAssociated {
    type ArrangedAccounts = CreateSupportMintAssociatedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let admin_group = accounts.get(1)?;
        let token_mint = accounts.get(2)?;
        let support_mint_associated = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(CreateSupportMintAssociatedInstructionAccounts {
            owner: owner.pubkey,
            admin_group: admin_group.pubkey,
            token_mint: token_mint.pubkey,
            support_mint_associated: support_mint_associated.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
