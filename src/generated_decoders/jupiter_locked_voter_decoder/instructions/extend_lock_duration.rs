use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb169c481998988e6")]
pub struct ExtendLockDuration {
    pub duration: i64,
}

pub struct ExtendLockDurationInstructionAccounts {
    pub locker: solana_sdk::pubkey::Pubkey,
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub escrow_owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ExtendLockDuration {
    type ArrangedAccounts = ExtendLockDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [locker, escrow, escrow_owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ExtendLockDurationInstructionAccounts {
            locker: locker.pubkey,
            escrow: escrow.pubkey,
            escrow_owner: escrow_owner.pubkey,
        })
    }
}
