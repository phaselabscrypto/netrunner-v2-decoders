



use super::UnstakeDecoder;
pub mod init_protocol_fee;
pub mod set_protocol_fee;
pub mod create_pool;
pub mod add_liquidity;
pub mod remove_liquidity;
pub mod set_fee;
pub mod set_fee_authority;
pub mod set_lp_token_metadata;
pub mod deactivate_stake_account;
pub mod reclaim_stake_account;
pub mod unstake;
pub mod unstake_wsol;
pub mod set_flash_loan_fee;
pub mod take_flash_loan;
pub mod repay_flash_loan;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum UnstakeInstruction {
    InitProtocolFee(init_protocol_fee::InitProtocolFee),
    SetProtocolFee(set_protocol_fee::SetProtocolFee),
    CreatePool(create_pool::CreatePool),
    AddLiquidity(add_liquidity::AddLiquidity),
    RemoveLiquidity(remove_liquidity::RemoveLiquidity),
    SetFee(set_fee::SetFee),
    SetFeeAuthority(set_fee_authority::SetFeeAuthority),
    SetLpTokenMetadata(set_lp_token_metadata::SetLpTokenMetadata),
    DeactivateStakeAccount(deactivate_stake_account::DeactivateStakeAccount),
    ReclaimStakeAccount(reclaim_stake_account::ReclaimStakeAccount),
    Unstake(unstake::Unstake),
    UnstakeWsol(unstake_wsol::UnstakeWsol),
    SetFlashLoanFee(set_flash_loan_fee::SetFlashLoanFee),
    TakeFlashLoan(take_flash_loan::TakeFlashLoan),
    RepayFlashLoan(repay_flash_loan::RepayFlashLoan),
}

impl carbon_core::instruction::InstructionDecoder<'_> for UnstakeDecoder {
    type InstructionType = UnstakeInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            UnstakeInstruction::InitProtocolFee => init_protocol_fee::InitProtocolFee,
            UnstakeInstruction::SetProtocolFee => set_protocol_fee::SetProtocolFee,
            UnstakeInstruction::CreatePool => create_pool::CreatePool,
            UnstakeInstruction::AddLiquidity => add_liquidity::AddLiquidity,
            UnstakeInstruction::RemoveLiquidity => remove_liquidity::RemoveLiquidity,
            UnstakeInstruction::SetFee => set_fee::SetFee,
            UnstakeInstruction::SetFeeAuthority => set_fee_authority::SetFeeAuthority,
            UnstakeInstruction::SetLpTokenMetadata => set_lp_token_metadata::SetLpTokenMetadata,
            UnstakeInstruction::DeactivateStakeAccount => deactivate_stake_account::DeactivateStakeAccount,
            UnstakeInstruction::ReclaimStakeAccount => reclaim_stake_account::ReclaimStakeAccount,
            UnstakeInstruction::Unstake => unstake::Unstake,
            UnstakeInstruction::UnstakeWsol => unstake_wsol::UnstakeWsol,
            UnstakeInstruction::SetFlashLoanFee => set_flash_loan_fee::SetFlashLoanFee,
            UnstakeInstruction::TakeFlashLoan => take_flash_loan::TakeFlashLoan,
            UnstakeInstruction::RepayFlashLoan => repay_flash_loan::RepayFlashLoan,
        )
    }
}