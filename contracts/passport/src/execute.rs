use cosmwasm_std::{DepsMut, Empty, Env, MessageInfo, Response};
use cw721_base::state::TokenInfo;
use cw721_base::{ContractError, Cw721Contract};

use crate::msg::Metadata;
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
