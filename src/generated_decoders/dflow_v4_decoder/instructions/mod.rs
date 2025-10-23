use super::SwapOrchestratorDecoder;
pub mod close_order;
pub mod create_referral_token_account_idempotent;
pub mod fee_event;
pub mod fill_order;
pub mod open_order;
pub mod swap;
pub mod swap2;
pub mod swap2_with_destination;
pub mod swap2_with_destination_native;
pub mod swap_event;
pub mod swap_with_destination;
pub mod swap_with_destination_native;
pub mod transfer_fee;
pub mod transfer_sol;
pub mod transfer_to_sponsor;
pub mod unwrap_sol;
pub mod wrap_sol;

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
pub enum SwapOrchestratorInstruction {
    CloseOrder(close_order::CloseOrder),
    CreateReferralTokenAccountIdempotent(
        create_referral_token_account_idempotent::CreateReferralTokenAccountIdempotent,
    ),
    FillOrder(fill_order::FillOrder),
    OpenOrder(open_order::OpenOrder),
    Swap(swap::Swap),
    Swap2(swap2::Swap2),
    Swap2WithDestination(swap2_with_destination::Swap2WithDestination),
    Swap2WithDestinationNative(swap2_with_destination_native::Swap2WithDestinationNative),
    SwapWithDestination(swap_with_destination::SwapWithDestination),
    SwapWithDestinationNative(swap_with_destination_native::SwapWithDestinationNative),
    TransferFee(transfer_fee::TransferFee),
    TransferSol(transfer_sol::TransferSol),
    TransferToSponsor(transfer_to_sponsor::TransferToSponsor),
    UnwrapSol(unwrap_sol::UnwrapSol),
    WrapSol(wrap_sol::WrapSol),
    FeeEvent(fee_event::FeeEvent),
    SwapEvent(swap_event::SwapEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for SwapOrchestratorDecoder {
    type InstructionType = SwapOrchestratorInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            SwapOrchestratorInstruction::CloseOrder => close_order::CloseOrder,
            SwapOrchestratorInstruction::CreateReferralTokenAccountIdempotent => create_referral_token_account_idempotent::CreateReferralTokenAccountIdempotent,
            SwapOrchestratorInstruction::FillOrder => fill_order::FillOrder,
            SwapOrchestratorInstruction::OpenOrder => open_order::OpenOrder,
            SwapOrchestratorInstruction::Swap => swap::Swap,
            SwapOrchestratorInstruction::Swap2 => swap2::Swap2,
            SwapOrchestratorInstruction::Swap2WithDestination => swap2_with_destination::Swap2WithDestination,
            SwapOrchestratorInstruction::Swap2WithDestinationNative => swap2_with_destination_native::Swap2WithDestinationNative,
            SwapOrchestratorInstruction::SwapWithDestination => swap_with_destination::SwapWithDestination,
            SwapOrchestratorInstruction::SwapWithDestinationNative => swap_with_destination_native::SwapWithDestinationNative,
            SwapOrchestratorInstruction::TransferFee => transfer_fee::TransferFee,
            SwapOrchestratorInstruction::TransferSol => transfer_sol::TransferSol,
            SwapOrchestratorInstruction::TransferToSponsor => transfer_to_sponsor::TransferToSponsor,
            SwapOrchestratorInstruction::UnwrapSol => unwrap_sol::UnwrapSol,
            SwapOrchestratorInstruction::WrapSol => wrap_sol::WrapSol,
            SwapOrchestratorInstruction::FeeEvent => fee_event::FeeEvent,
            SwapOrchestratorInstruction::SwapEvent => swap_event::SwapEvent,
        )
    }
}
