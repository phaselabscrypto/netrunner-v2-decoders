// use super::*;

use carbon_core::{borsh, CarbonDeserialize};
use mpl_core::types::PluginAuthorityPair;

#[derive(CarbonDeserialize, Debug, PartialEq, Eq, Clone)]
pub struct MintAssetArgs {
    pub plugins: Vec<PluginAuthorityPair>,
}
