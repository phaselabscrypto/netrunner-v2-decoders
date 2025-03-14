



use super::TensorswapDecoder;
pub mod init_update_tswap;
pub mod init_pool;
pub mod close_pool;
pub mod deposit_nft;
pub mod withdraw_nft;
pub mod deposit_sol;
pub mod withdraw_sol;
pub mod buy_nft;
pub mod sell_nft_token_pool;
pub mod sell_nft_trade_pool;
pub mod edit_pool;
pub mod realloc_pool;
pub mod init_margin_account;
pub mod close_margin_account;
pub mod deposit_margin_account;
pub mod withdraw_margin_account;
pub mod attach_pool_to_margin;
pub mod detach_pool_from_margin;
pub mod set_pool_freeze;
pub mod take_snipe;
pub mod edit_pool_in_place;
pub mod withdraw_tswap_fees;
pub mod list;
pub mod delist;
pub mod buy_single_listing;
pub mod edit_single_listing;
pub mod withdraw_mm_fee;
pub mod withdraw_margin_account_cpi;
pub mod withdraw_margin_account_cpi_tcomp;
pub mod withdraw_margin_account_cpi_tlock;
pub mod buy_nft_t22;
pub mod deposit_nft_t22;
pub mod sell_nft_token_pool_t22;
pub mod sell_nft_trade_pool_t22;
pub mod withdraw_nft_t22;
pub mod buy_single_listing_t22;
pub mod list_t22;
pub mod delist_t22;
pub mod wns_buy_nft;
pub mod wns_deposit_nft;
pub mod wns_sell_nft_token_pool;
pub mod wns_sell_nft_trade_pool;
pub mod wns_withdraw_nft;
pub mod wns_buy_single_listing;
pub mod wns_list;
pub mod wns_delist;
pub mod buy_sell_event;
pub mod delist_event;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum TensorswapInstruction {
    InitUpdateTswap(init_update_tswap::InitUpdateTswap),
    InitPool(init_pool::InitPool),
    ClosePool(close_pool::ClosePool),
    DepositNft(deposit_nft::DepositNft),
    WithdrawNft(withdraw_nft::WithdrawNft),
    DepositSol(deposit_sol::DepositSol),
    WithdrawSol(withdraw_sol::WithdrawSol),
    BuyNft(buy_nft::BuyNft),
    SellNftTokenPool(sell_nft_token_pool::SellNftTokenPool),
    SellNftTradePool(sell_nft_trade_pool::SellNftTradePool),
    EditPool(edit_pool::EditPool),
    ReallocPool(realloc_pool::ReallocPool),
    InitMarginAccount(init_margin_account::InitMarginAccount),
    CloseMarginAccount(close_margin_account::CloseMarginAccount),
    DepositMarginAccount(deposit_margin_account::DepositMarginAccount),
    WithdrawMarginAccount(withdraw_margin_account::WithdrawMarginAccount),
    AttachPoolToMargin(attach_pool_to_margin::AttachPoolToMargin),
    DetachPoolFromMargin(detach_pool_from_margin::DetachPoolFromMargin),
    SetPoolFreeze(set_pool_freeze::SetPoolFreeze),
    TakeSnipe(take_snipe::TakeSnipe),
    EditPoolInPlace(edit_pool_in_place::EditPoolInPlace),
    WithdrawTswapFees(withdraw_tswap_fees::WithdrawTswapFees),
    List(list::List),
    Delist(delist::Delist),
    BuySingleListing(buy_single_listing::BuySingleListing),
    EditSingleListing(edit_single_listing::EditSingleListing),
    WithdrawMmFee(withdraw_mm_fee::WithdrawMmFee),
    WithdrawMarginAccountCpi(withdraw_margin_account_cpi::WithdrawMarginAccountCpi),
    WithdrawMarginAccountCpiTcomp(withdraw_margin_account_cpi_tcomp::WithdrawMarginAccountCpiTcomp),
    WithdrawMarginAccountCpiTlock(withdraw_margin_account_cpi_tlock::WithdrawMarginAccountCpiTlock),
    BuyNftT22(buy_nft_t22::BuyNftT22),
    DepositNftT22(deposit_nft_t22::DepositNftT22),
    SellNftTokenPoolT22(sell_nft_token_pool_t22::SellNftTokenPoolT22),
    SellNftTradePoolT22(sell_nft_trade_pool_t22::SellNftTradePoolT22),
    WithdrawNftT22(withdraw_nft_t22::WithdrawNftT22),
    BuySingleListingT22(buy_single_listing_t22::BuySingleListingT22),
    ListT22(list_t22::ListT22),
    DelistT22(delist_t22::DelistT22),
    WnsBuyNft(wns_buy_nft::WnsBuyNft),
    WnsDepositNft(wns_deposit_nft::WnsDepositNft),
    WnsSellNftTokenPool(wns_sell_nft_token_pool::WnsSellNftTokenPool),
    WnsSellNftTradePool(wns_sell_nft_trade_pool::WnsSellNftTradePool),
    WnsWithdrawNft(wns_withdraw_nft::WnsWithdrawNft),
    WnsBuySingleListing(wns_buy_single_listing::WnsBuySingleListing),
    WnsList(wns_list::WnsList),
    WnsDelist(wns_delist::WnsDelist),
    BuySellEvent(buy_sell_event::BuySellEvent),
    DelistEvent(delist_event::DelistEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for TensorswapDecoder {
    type InstructionType = TensorswapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            TensorswapInstruction::InitUpdateTswap => init_update_tswap::InitUpdateTswap,
            TensorswapInstruction::InitPool => init_pool::InitPool,
            TensorswapInstruction::ClosePool => close_pool::ClosePool,
            TensorswapInstruction::DepositNft => deposit_nft::DepositNft,
            TensorswapInstruction::WithdrawNft => withdraw_nft::WithdrawNft,
            TensorswapInstruction::DepositSol => deposit_sol::DepositSol,
            TensorswapInstruction::WithdrawSol => withdraw_sol::WithdrawSol,
            TensorswapInstruction::BuyNft => buy_nft::BuyNft,
            TensorswapInstruction::SellNftTokenPool => sell_nft_token_pool::SellNftTokenPool,
            TensorswapInstruction::SellNftTradePool => sell_nft_trade_pool::SellNftTradePool,
            TensorswapInstruction::EditPool => edit_pool::EditPool,
            TensorswapInstruction::ReallocPool => realloc_pool::ReallocPool,
            TensorswapInstruction::InitMarginAccount => init_margin_account::InitMarginAccount,
            TensorswapInstruction::CloseMarginAccount => close_margin_account::CloseMarginAccount,
            TensorswapInstruction::DepositMarginAccount => deposit_margin_account::DepositMarginAccount,
            TensorswapInstruction::WithdrawMarginAccount => withdraw_margin_account::WithdrawMarginAccount,
            TensorswapInstruction::AttachPoolToMargin => attach_pool_to_margin::AttachPoolToMargin,
            TensorswapInstruction::DetachPoolFromMargin => detach_pool_from_margin::DetachPoolFromMargin,
            TensorswapInstruction::SetPoolFreeze => set_pool_freeze::SetPoolFreeze,
            TensorswapInstruction::TakeSnipe => take_snipe::TakeSnipe,
            TensorswapInstruction::EditPoolInPlace => edit_pool_in_place::EditPoolInPlace,
            TensorswapInstruction::WithdrawTswapFees => withdraw_tswap_fees::WithdrawTswapFees,
            TensorswapInstruction::List => list::List,
            TensorswapInstruction::Delist => delist::Delist,
            TensorswapInstruction::BuySingleListing => buy_single_listing::BuySingleListing,
            TensorswapInstruction::EditSingleListing => edit_single_listing::EditSingleListing,
            TensorswapInstruction::WithdrawMmFee => withdraw_mm_fee::WithdrawMmFee,
            TensorswapInstruction::WithdrawMarginAccountCpi => withdraw_margin_account_cpi::WithdrawMarginAccountCpi,
            TensorswapInstruction::WithdrawMarginAccountCpiTcomp => withdraw_margin_account_cpi_tcomp::WithdrawMarginAccountCpiTcomp,
            TensorswapInstruction::WithdrawMarginAccountCpiTlock => withdraw_margin_account_cpi_tlock::WithdrawMarginAccountCpiTlock,
            TensorswapInstruction::BuyNftT22 => buy_nft_t22::BuyNftT22,
            TensorswapInstruction::DepositNftT22 => deposit_nft_t22::DepositNftT22,
            TensorswapInstruction::SellNftTokenPoolT22 => sell_nft_token_pool_t22::SellNftTokenPoolT22,
            TensorswapInstruction::SellNftTradePoolT22 => sell_nft_trade_pool_t22::SellNftTradePoolT22,
            TensorswapInstruction::WithdrawNftT22 => withdraw_nft_t22::WithdrawNftT22,
            TensorswapInstruction::BuySingleListingT22 => buy_single_listing_t22::BuySingleListingT22,
            TensorswapInstruction::ListT22 => list_t22::ListT22,
            TensorswapInstruction::DelistT22 => delist_t22::DelistT22,
            TensorswapInstruction::WnsBuyNft => wns_buy_nft::WnsBuyNft,
            TensorswapInstruction::WnsDepositNft => wns_deposit_nft::WnsDepositNft,
            TensorswapInstruction::WnsSellNftTokenPool => wns_sell_nft_token_pool::WnsSellNftTokenPool,
            TensorswapInstruction::WnsSellNftTradePool => wns_sell_nft_trade_pool::WnsSellNftTradePool,
            TensorswapInstruction::WnsWithdrawNft => wns_withdraw_nft::WnsWithdrawNft,
            TensorswapInstruction::WnsBuySingleListing => wns_buy_single_listing::WnsBuySingleListing,
            TensorswapInstruction::WnsList => wns_list::WnsList,
            TensorswapInstruction::WnsDelist => wns_delist::WnsDelist,
            TensorswapInstruction::BuySellEvent => buy_sell_event::BuySellEvent,
            TensorswapInstruction::DelistEvent => delist_event::DelistEvent,
        )
    }
}