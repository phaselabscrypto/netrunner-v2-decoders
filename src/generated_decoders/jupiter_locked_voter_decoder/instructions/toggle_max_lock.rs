use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa39da184b36b7f8f")]
pub struct ToggleMaxLock {
    pub is_max_lock: bool,
}

pub struct ToggleMaxLockInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ToggleMaxLock {
    type ArrangedAccounts = ToggleMaxLockInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, escrow_owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ToggleMaxLockInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            escrow_owner: escrow_owner.pubkey,
        })
    }
}
