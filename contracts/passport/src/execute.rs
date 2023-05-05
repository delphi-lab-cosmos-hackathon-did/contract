use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response, StdError, Uint128};
use cw721_base::state::TokenInfo;
use cw721_base::{ContractError, Cw721Contract};

use crate::msg::{Metadata, Trait};
use crate::state::{badges, BadgeInfo, CONFIG};
type Extension = Option<Metadata>;

pub fn mint(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
    let minter = info.sender;
    let token_id = minter.to_string();
    // load token
    let token = tract.tokens.may_load(deps.storage, &token_id)?;
    match token {
        Some(_) => return Err(ContractError::Claimed {}),
        None => {}
    }

    // create the token
    let token_uri = None;
    let extension = Some(Metadata {
        image: None,
        image_data: None,
        external_url: None,
        description: None,
        name: None,
        attributes: None,
        background_color: None,
        animation_url: None,
        youtube_url: None,
    });
    let token = TokenInfo {
        owner: deps.api.addr_validate(&minter.to_string())?,
        approvals: vec![],
        token_uri,
        extension,
    };
    tract.tokens.save(deps.storage, &token_id, &token)?;
    tract.increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", minter.to_string())
        .add_attribute("owner", minter.to_string())
        .add_attribute("token_id", token_id))
}

pub fn issue(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    category: String,
    badge: String,
    owner: String,
) -> Result<Response, ContractError> {
    // check admin
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.admin {
        return Err(ContractError::Ownership(
            cw721_base::OwnershipError::NotOwner,
        ));
    }
    // load existing badge (using key as category_badge_owner)
    let badge_key = _badge_key(&category, &badge, &owner);
    // check existing issue
    let badge_info = match badges().may_load(deps.storage, &badge_key.clone())? {
        Some(b) => b,
        None => BadgeInfo {
            owner: deps.api.addr_validate(&owner)?,
            category: category.clone(),
            badge: badge.clone(),
            is_claimed: false,
            issue_time: Uint128::from(env.block.time.seconds()),
            claimed_time: Uint128::zero(),
        },
    };
    badges().save(deps.storage, &badge_key, &badge_info)?;
    Ok(Response::new()
        .add_attribute("action", "issue")
        .add_attribute("category", category)
        .add_attribute("badge", badge)
        .add_attribute("owner", owner))
}

pub fn claim(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    category: String,
    badge: String,
    owner: String,
) -> Result<Response, ContractError> {
    // load token
    let token_id = owner.clone();
    let tract = Cw721Contract::<Extension, Empty, Empty, Empty>::default();
    let mut token = tract.tokens.load(deps.storage, &token_id)?;
    // check is claimed
    let badge_key = _badge_key(&category, &badge, &owner);
    let mut badge_info = badges().load(deps.storage, &badge_key)?;
    if badge_info.is_claimed {
        return Err(ContractError::Claimed {});
    }
    // load trait & update token
    let mut extension = match token.extension {
        Some(e) => e,
        None => {
            return Err(ContractError::Std(StdError::NotFound {
                kind: "No extension data".to_string(),
            }))
        }
    };
    let mut attributes = match extension.attributes {
        Some(a) => a,
        None => vec![],
    };
    let mut value = category.to_owned();
    value.push_str("_");
    value.push_str(&badge);
    attributes.push(Trait {
        display_type: None,
        trait_type: value.clone(),
        value: value.clone(),
    });
    extension.attributes = Some(attributes);
    token.extension = Some(extension);
    tract.tokens.save(deps.storage, &token_id, &token)?;
    // update badge
    badge_info.is_claimed = true;
    badge_info.claimed_time = Uint128::from(env.block.time.seconds());
    badges().save(deps.storage, &badge_key, &badge_info)?;
    Ok(Response::new()
        .add_attribute("action", "claim")
        .add_attribute("category", category)
        .add_attribute("badge", badge)
        .add_attribute("owner", owner))
}

fn _badge_key(category: &str, badge: &str, owner: &str) -> String {
    let mut badge_key = category.to_owned();
    badge_key.push_str("_");
    badge_key.push_str(badge);
    badge_key.push_str("_");
    badge_key.push_str(owner);
    return badge_key.clone();
}
