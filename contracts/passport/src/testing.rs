use crate::entry::{execute, instantiate};
use crate::msg::{BadgeResponse, BadgesResponse, ExecuteMsg, InstantiateMsg};
use crate::query::badges_owner;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{DepsMut, Timestamp, Uint128};

fn setup_contract(deps: DepsMut) {
    let msg = InstantiateMsg {
        admin: "admin".to_string(),
        name: "passport".to_string(),
        symbol: "PP".to_string(),
    };
    let info = mock_info("admin", &[]);
    let env = mock_env();
    let res = instantiate(deps, env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());
}

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());
}

#[test]
fn issue() {
    let mut deps = mock_dependencies();
    setup_contract(deps.as_mut());

    // issue new badge
    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(100);
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::Issue {
        category: "osmo".to_string(),
        badge: "whale".to_string(),
        owner: "whale1".to_string(),
    };
    execute(deps.as_mut(), env, info, msg).unwrap();
    // query badge by owner
    let result = badges_owner(deps.as_ref(), "whale1".to_string(), None, None).unwrap();
    assert_eq!(
        result,
        BadgesResponse {
            badges: vec![BadgeResponse {
                key: "osmo_whale_whale1".to_string(),
                owner: "whale1".to_string(),
                category: "osmo".to_string(),
                badge: "whale".to_string(),
                is_claimed: false,
                issue_time: Uint128::from(100u128),
                claimed_time: Uint128::zero()
            }]
        }
    );
    let result = badges_owner(
        deps.as_ref(),
        "whale1".to_string(),
        Some("osmo_whale_whale1".to_string()),
        None,
    )
    .unwrap();
    assert_eq!(result, BadgesResponse { badges: vec![] });
}
