use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfd764225be319a66")]
pub struct SetTreeDelegate {}

pub struct SetTreeDelegateInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub tree_creator: solana_sdk::pubkey::Pubkey,
    pub new_tree_delegate: solana_sdk::pubkey::Pubkey,
    pub merkle_tree: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetTreeDelegate {
    type ArrangedAccounts = SetTreeDelegateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let tree_creator = accounts.get(1)?;
        let new_tree_delegate = accounts.get(2)?;
        let merkle_tree = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(SetTreeDelegateInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            tree_creator: tree_creator.pubkey,
            new_tree_delegate: new_tree_delegate.pubkey,
            merkle_tree: merkle_tree.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
