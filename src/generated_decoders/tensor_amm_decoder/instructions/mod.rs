use super::AmmProgramDecoder;
pub mod buy_nft;
pub mod buy_nft_core;
pub mod buy_nft_t22;
pub mod close_expired_pool;
pub mod close_pool;
pub mod create_pool;
pub mod deposit_nft;
pub mod deposit_nft_core;
pub mod deposit_nft_t22;
pub mod deposit_sol;
pub mod edit_pool;
pub mod sell_nft_token_pool;
pub mod sell_nft_token_pool_core;
pub mod sell_nft_token_pool_t22;
pub mod sell_nft_trade_pool;
pub mod sell_nft_trade_pool_core;
pub mod sell_nft_trade_pool_t22;
pub mod tamm_noop;
pub mod withdraw_nft;
pub mod withdraw_nft_core;
pub mod withdraw_nft_t22;
pub mod withdraw_sol;

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
pub enum AmmProgramInstruction {
    TammNoop(tamm_noop::TammNoop),
    CreatePool(create_pool::CreatePool),
    EditPool(edit_pool::EditPool),
    ClosePool(close_pool::ClosePool),
    CloseExpiredPool(close_expired_pool::CloseExpiredPool),
    DepositSol(deposit_sol::DepositSol),
    WithdrawSol(withdraw_sol::WithdrawSol),
    DepositNft(deposit_nft::DepositNft),
    WithdrawNft(withdraw_nft::WithdrawNft),
    BuyNft(buy_nft::BuyNft),
    SellNftTokenPool(sell_nft_token_pool::SellNftTokenPool),
    SellNftTradePool(sell_nft_trade_pool::SellNftTradePool),
    DepositNftCore(deposit_nft_core::DepositNftCore),
    WithdrawNftCore(withdraw_nft_core::WithdrawNftCore),
    BuyNftCore(buy_nft_core::BuyNftCore),
    SellNftTokenPoolCore(sell_nft_token_pool_core::SellNftTokenPoolCore),
    SellNftTradePoolCore(sell_nft_trade_pool_core::SellNftTradePoolCore),
    DepositNftT22(deposit_nft_t22::DepositNftT22),
    WithdrawNftT22(withdraw_nft_t22::WithdrawNftT22),
    BuyNftT22(buy_nft_t22::BuyNftT22),
    SellNftTokenPoolT22(sell_nft_token_pool_t22::SellNftTokenPoolT22),
    SellNftTradePoolT22(sell_nft_trade_pool_t22::SellNftTradePoolT22),
}

impl carbon_core::instruction::InstructionDecoder<'_> for AmmProgramDecoder {
    type InstructionType = AmmProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            AmmProgramInstruction::TammNoop => tamm_noop::TammNoop,
            AmmProgramInstruction::CreatePool => create_pool::CreatePool,
            AmmProgramInstruction::EditPool => edit_pool::EditPool,
            AmmProgramInstruction::ClosePool => close_pool::ClosePool,
            AmmProgramInstruction::CloseExpiredPool => close_expired_pool::CloseExpiredPool,
            AmmProgramInstruction::DepositSol => deposit_sol::DepositSol,
            AmmProgramInstruction::WithdrawSol => withdraw_sol::WithdrawSol,
            AmmProgramInstruction::DepositNft => deposit_nft::DepositNft,
            AmmProgramInstruction::WithdrawNft => withdraw_nft::WithdrawNft,
            AmmProgramInstruction::BuyNft => buy_nft::BuyNft,
            AmmProgramInstruction::SellNftTokenPool => sell_nft_token_pool::SellNftTokenPool,
            AmmProgramInstruction::SellNftTradePool => sell_nft_trade_pool::SellNftTradePool,
            AmmProgramInstruction::DepositNftCore => deposit_nft_core::DepositNftCore,
            AmmProgramInstruction::WithdrawNftCore => withdraw_nft_core::WithdrawNftCore,
            AmmProgramInstruction::BuyNftCore => buy_nft_core::BuyNftCore,
            AmmProgramInstruction::SellNftTokenPoolCore => sell_nft_token_pool_core::SellNftTokenPoolCore,
            AmmProgramInstruction::SellNftTradePoolCore => sell_nft_trade_pool_core::SellNftTradePoolCore,
            AmmProgramInstruction::DepositNftT22 => deposit_nft_t22::DepositNftT22,
            AmmProgramInstruction::WithdrawNftT22 => withdraw_nft_t22::WithdrawNftT22,
            AmmProgramInstruction::BuyNftT22 => buy_nft_t22::BuyNftT22,
            AmmProgramInstruction::SellNftTokenPoolT22 => sell_nft_token_pool_t22::SellNftTokenPoolT22,
            AmmProgramInstruction::SellNftTradePoolT22 => sell_nft_trade_pool_t22::SellNftTradePoolT22,
        )
    }
}
