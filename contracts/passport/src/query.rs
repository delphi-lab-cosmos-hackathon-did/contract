use crate::{
    msg::{AdminResponse, BadgesResponse},
    state::{badges, BadgeInfo, CONFIG},
};
use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(AdminResponse {
        admin: config.admin.to_string(),
    })
}

pub fn badges_owner(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<BadgesResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start: Option<Bound<String>> = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let owner_addr = deps.api.addr_validate(&owner)?;
    let badges: Vec<BadgeInfo> = badges()
        .idx
        .owner
        .prefix(owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|x| {
            let a = x.unwrap();
            return a.1;
        })
        .collect();
    Ok(BadgesResponse { badges })
}
