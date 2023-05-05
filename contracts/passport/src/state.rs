use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Config {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BadgeInfo {
    pub owner: Addr,
    pub category: String,
    pub badge: String,
    pub is_claimed: bool,
    pub issue_time: Uint128,
    pub claimed_time: Uint128,
}

pub struct BadgeIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, BadgeInfo, String>,
    // pub category_badge: MultiIndex<'a, (String, String), BadgeInfo, String>,
}

impl<'a> IndexList<BadgeInfo> for BadgeIndexes<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<BadgeInfo>> + '_> {
        let v: Vec<&dyn Index<BadgeInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn badges<'a>() -> IndexedMap<'a, &'a str, BadgeInfo, BadgeIndexes<'a>> {
    let indexes = BadgeIndexes {
        owner: MultiIndex::new(|_pk, b| b.owner.clone(), "badge", "badge__owner"),
    };
    IndexedMap::new("badges", indexes)
}

pub const CONFIG: Item<Config> = Item::new("config");
