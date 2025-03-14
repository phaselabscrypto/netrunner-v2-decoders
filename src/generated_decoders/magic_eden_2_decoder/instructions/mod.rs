



use super::M2Decoder;
pub mod withdraw_from_treasury;
pub mod update_auction_house;
pub mod create_auction_house;
pub mod withdraw;
pub mod deposit;
pub mod sell;
pub mod cancel_sell;
pub mod buy;
pub mod buy_v2;
pub mod cancel_buy;
pub mod ocp_sell;
pub mod ocp_cancel_sell;
pub mod ocp_execute_sale_v2;
pub mod execute_sale_v2;
pub mod mip1_sell;
pub mod mip1_execute_sale_v2;
pub mod mip1_cancel_sell;
pub mod withdraw_by_mmm;
pub mod ext_sell;
pub mod ext_execute_sale_v2;
pub mod ext_cancel_sell;
pub mod core_sell;
pub mod core_cancel_sell;
pub mod core_execute_sale_v2;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum M2Instruction {
    WithdrawFromTreasury(withdraw_from_treasury::WithdrawFromTreasury),
    UpdateAuctionHouse(update_auction_house::UpdateAuctionHouse),
    CreateAuctionHouse(create_auction_house::CreateAuctionHouse),
    Withdraw(withdraw::Withdraw),
    Deposit(deposit::Deposit),
    Sell(sell::Sell),
    CancelSell(cancel_sell::CancelSell),
    Buy(buy::Buy),
    BuyV2(buy_v2::BuyV2),
    CancelBuy(cancel_buy::CancelBuy),
    OcpSell(ocp_sell::OcpSell),
    OcpCancelSell(ocp_cancel_sell::OcpCancelSell),
    OcpExecuteSaleV2(ocp_execute_sale_v2::OcpExecuteSaleV2),
    ExecuteSaleV2(execute_sale_v2::ExecuteSaleV2),
    Mip1Sell(mip1_sell::Mip1Sell),
    Mip1ExecuteSaleV2(mip1_execute_sale_v2::Mip1ExecuteSaleV2),
    Mip1CancelSell(mip1_cancel_sell::Mip1CancelSell),
    WithdrawByMmm(withdraw_by_mmm::WithdrawByMmm),
    ExtSell(ext_sell::ExtSell),
    ExtExecuteSaleV2(ext_execute_sale_v2::ExtExecuteSaleV2),
    ExtCancelSell(ext_cancel_sell::ExtCancelSell),
    CoreSell(core_sell::CoreSell),
    CoreCancelSell(core_cancel_sell::CoreCancelSell),
    CoreExecuteSaleV2(core_execute_sale_v2::CoreExecuteSaleV2),
}

impl carbon_core::instruction::InstructionDecoder<'_> for M2Decoder {
    type InstructionType = M2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            M2Instruction::WithdrawFromTreasury => withdraw_from_treasury::WithdrawFromTreasury,
            M2Instruction::UpdateAuctionHouse => update_auction_house::UpdateAuctionHouse,
            M2Instruction::CreateAuctionHouse => create_auction_house::CreateAuctionHouse,
            M2Instruction::Withdraw => withdraw::Withdraw,
            M2Instruction::Deposit => deposit::Deposit,
            M2Instruction::Sell => sell::Sell,
            M2Instruction::CancelSell => cancel_sell::CancelSell,
            M2Instruction::Buy => buy::Buy,
            M2Instruction::BuyV2 => buy_v2::BuyV2,
            M2Instruction::CancelBuy => cancel_buy::CancelBuy,
            M2Instruction::OcpSell => ocp_sell::OcpSell,
            M2Instruction::OcpCancelSell => ocp_cancel_sell::OcpCancelSell,
            M2Instruction::OcpExecuteSaleV2 => ocp_execute_sale_v2::OcpExecuteSaleV2,
            M2Instruction::ExecuteSaleV2 => execute_sale_v2::ExecuteSaleV2,
            M2Instruction::Mip1Sell => mip1_sell::Mip1Sell,
            M2Instruction::Mip1ExecuteSaleV2 => mip1_execute_sale_v2::Mip1ExecuteSaleV2,
            M2Instruction::Mip1CancelSell => mip1_cancel_sell::Mip1CancelSell,
            M2Instruction::WithdrawByMmm => withdraw_by_mmm::WithdrawByMmm,
            M2Instruction::ExtSell => ext_sell::ExtSell,
            M2Instruction::ExtExecuteSaleV2 => ext_execute_sale_v2::ExtExecuteSaleV2,
            M2Instruction::ExtCancelSell => ext_cancel_sell::ExtCancelSell,
            M2Instruction::CoreSell => core_sell::CoreSell,
            M2Instruction::CoreCancelSell => core_cancel_sell::CoreCancelSell,
            M2Instruction::CoreExecuteSaleV2 => core_execute_sale_v2::CoreExecuteSaleV2,
        )
    }
}