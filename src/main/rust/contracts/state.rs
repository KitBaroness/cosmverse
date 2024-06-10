use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub fee_accumulator: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftInfo {
    pub owner: Addr,
    pub price: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const NFTS: Map<String, NftInfo> = Map::new("nfts");
