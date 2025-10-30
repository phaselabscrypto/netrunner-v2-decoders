use super::LockedVoterDecoder;
pub mod activate_proposal;
pub mod cast_vote;
pub mod exit_escrow_event;
pub mod extend_lock_duration;
pub mod extend_lock_duration_event;
pub mod increase_locked_amount;
pub mod increase_locked_amount_event;
pub mod locker_set_params_event;
pub mod merge_partial_unstaking;
pub mod merge_partial_unstaking_event;
pub mod new_escrow;
pub mod new_escrow_event;
pub mod new_locker;
pub mod new_locker_event;
pub mod open_partial_staking_event;
pub mod open_partial_unstaking;
pub mod set_locker_params;
pub mod set_vote_delegate;
pub mod set_vote_delegate_event;
pub mod toggle_max_lock;
pub mod withdraw;
pub mod withdraw_partial_unstaking;
pub mod withdraw_partial_unstaking_event;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum LockedVoterInstruction {
    NewLocker(new_locker::NewLocker),
    NewEscrow(new_escrow::NewEscrow),
    IncreaseLockedAmount(increase_locked_amount::IncreaseLockedAmount),
    ExtendLockDuration(extend_lock_duration::ExtendLockDuration),
    ToggleMaxLock(toggle_max_lock::ToggleMaxLock),
    Withdraw(withdraw::Withdraw),
    ActivateProposal(activate_proposal::ActivateProposal),
    CastVote(cast_vote::CastVote),
    SetVoteDelegate(set_vote_delegate::SetVoteDelegate),
    SetLockerParams(set_locker_params::SetLockerParams),
    OpenPartialUnstaking(open_partial_unstaking::OpenPartialUnstaking),
    MergePartialUnstaking(merge_partial_unstaking::MergePartialUnstaking),
    WithdrawPartialUnstaking(withdraw_partial_unstaking::WithdrawPartialUnstaking),
    ExtendLockDurationEvent(extend_lock_duration_event::ExtendLockDurationEvent),
    IncreaseLockedAmountEvent(increase_locked_amount_event::IncreaseLockedAmountEvent),
    MergePartialUnstakingEvent(merge_partial_unstaking_event::MergePartialUnstakingEvent),
    NewEscrowEvent(new_escrow_event::NewEscrowEvent),
    NewLockerEvent(new_locker_event::NewLockerEvent),
    OpenPartialStakingEvent(open_partial_staking_event::OpenPartialStakingEvent),
    LockerSetParamsEvent(locker_set_params_event::LockerSetParamsEvent),
    SetVoteDelegateEvent(set_vote_delegate_event::SetVoteDelegateEvent),
    WithdrawPartialUnstakingEvent(withdraw_partial_unstaking_event::WithdrawPartialUnstakingEvent),
    ExitEscrowEvent(exit_escrow_event::ExitEscrowEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for LockedVoterDecoder {
    type InstructionType = LockedVoterInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            LockedVoterInstruction::NewLocker => new_locker::NewLocker,
            LockedVoterInstruction::NewEscrow => new_escrow::NewEscrow,
            LockedVoterInstruction::IncreaseLockedAmount => increase_locked_amount::IncreaseLockedAmount,
            LockedVoterInstruction::ExtendLockDuration => extend_lock_duration::ExtendLockDuration,
            LockedVoterInstruction::ToggleMaxLock => toggle_max_lock::ToggleMaxLock,
            LockedVoterInstruction::Withdraw => withdraw::Withdraw,
            LockedVoterInstruction::ActivateProposal => activate_proposal::ActivateProposal,
            LockedVoterInstruction::CastVote => cast_vote::CastVote,
            LockedVoterInstruction::SetVoteDelegate => set_vote_delegate::SetVoteDelegate,
            LockedVoterInstruction::SetLockerParams => set_locker_params::SetLockerParams,
            LockedVoterInstruction::OpenPartialUnstaking => open_partial_unstaking::OpenPartialUnstaking,
            LockedVoterInstruction::MergePartialUnstaking => merge_partial_unstaking::MergePartialUnstaking,
            LockedVoterInstruction::WithdrawPartialUnstaking => withdraw_partial_unstaking::WithdrawPartialUnstaking,
            LockedVoterInstruction::ExtendLockDurationEvent => extend_lock_duration_event::ExtendLockDurationEvent,
            LockedVoterInstruction::IncreaseLockedAmountEvent => increase_locked_amount_event::IncreaseLockedAmountEvent,
            LockedVoterInstruction::MergePartialUnstakingEvent => merge_partial_unstaking_event::MergePartialUnstakingEvent,
            LockedVoterInstruction::NewEscrowEvent => new_escrow_event::NewEscrowEvent,
            LockedVoterInstruction::NewLockerEvent => new_locker_event::NewLockerEvent,
            LockedVoterInstruction::OpenPartialStakingEvent => open_partial_staking_event::OpenPartialStakingEvent,
            LockedVoterInstruction::LockerSetParamsEvent => locker_set_params_event::LockerSetParamsEvent,
            LockedVoterInstruction::SetVoteDelegateEvent => set_vote_delegate_event::SetVoteDelegateEvent,
            LockedVoterInstruction::WithdrawPartialUnstakingEvent => withdraw_partial_unstaking_event::WithdrawPartialUnstakingEvent,
            LockedVoterInstruction::ExitEscrowEvent => exit_escrow_event::ExitEscrowEvent,
        )
    }
}
