use super::TcompDecoder;
pub mod bid;
pub mod buy;
pub mod buy_core;
pub mod buy_spl;
pub mod cancel_bid;
pub mod close_expired_bid;
pub mod close_expired_listing;
pub mod close_expired_listing_core;
pub mod delist;
pub mod delist_core;
pub mod edit;
pub mod list;
pub mod list_core;
pub mod take_bid_core;
pub mod take_bid_full_meta;
pub mod take_bid_legacy;
pub mod take_bid_meta_hash;
pub mod take_bid_t22;
pub mod take_bid_wns;
pub mod tcomp_noop;
pub mod withdraw_fees;

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
pub enum TcompInstruction {
    TcompNoop(tcomp_noop::TcompNoop),
    WithdrawFees(withdraw_fees::WithdrawFees),
    Buy(buy::Buy),
    BuySpl(buy_spl::BuySpl),
    BuyCore(buy_core::BuyCore),
    List(list::List),
    Delist(delist::Delist),
    Edit(edit::Edit),
    ListCore(list_core::ListCore),
    DelistCore(delist_core::DelistCore),
    Bid(bid::Bid),
    CancelBid(cancel_bid::CancelBid),
    CloseExpiredBid(close_expired_bid::CloseExpiredBid),
    CloseExpiredListing(close_expired_listing::CloseExpiredListing),
    CloseExpiredListingCore(close_expired_listing_core::CloseExpiredListingCore),
    TakeBidMetaHash(take_bid_meta_hash::TakeBidMetaHash),
    TakeBidFullMeta(take_bid_full_meta::TakeBidFullMeta),
    TakeBidLegacy(take_bid_legacy::TakeBidLegacy),
    TakeBidT22(take_bid_t22::TakeBidT22),
    TakeBidWns(take_bid_wns::TakeBidWns),
    TakeBidCore(take_bid_core::TakeBidCore),
}

impl carbon_core::instruction::InstructionDecoder<'_> for TcompDecoder {
    type InstructionType = TcompInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            TcompInstruction::TcompNoop => tcomp_noop::TcompNoop,
            TcompInstruction::WithdrawFees => withdraw_fees::WithdrawFees,
            TcompInstruction::Buy => buy::Buy,
            TcompInstruction::BuySpl => buy_spl::BuySpl,
            TcompInstruction::BuyCore => buy_core::BuyCore,
            TcompInstruction::List => list::List,
            TcompInstruction::Delist => delist::Delist,
            TcompInstruction::Edit => edit::Edit,
            TcompInstruction::ListCore => list_core::ListCore,
            TcompInstruction::DelistCore => delist_core::DelistCore,
            TcompInstruction::Bid => bid::Bid,
            TcompInstruction::CancelBid => cancel_bid::CancelBid,
            TcompInstruction::CloseExpiredBid => close_expired_bid::CloseExpiredBid,
            TcompInstruction::CloseExpiredListing => close_expired_listing::CloseExpiredListing,
            TcompInstruction::CloseExpiredListingCore => close_expired_listing_core::CloseExpiredListingCore,
            TcompInstruction::TakeBidMetaHash => take_bid_meta_hash::TakeBidMetaHash,
            TcompInstruction::TakeBidFullMeta => take_bid_full_meta::TakeBidFullMeta,
            TcompInstruction::TakeBidLegacy => take_bid_legacy::TakeBidLegacy,
            TcompInstruction::TakeBidT22 => take_bid_t22::TakeBidT22,
            TcompInstruction::TakeBidWns => take_bid_wns::TakeBidWns,
            TcompInstruction::TakeBidCore => take_bid_core::TakeBidCore,
        )
    }
}
