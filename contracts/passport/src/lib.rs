pub use crate::msg::{InstantiateMsg, QueryMsg};
use cosmwasm_std::Empty;
pub use cw721_base::{
    ContractError, Cw721Contract, InstantiateMsg as Cw721BaseInstantiateMsg, MinterResponse,
};

pub mod execute;
pub mod msg;
pub mod query;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:passport-nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Extension = Option<msg::Metadata>;

pub type PassportContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {

    use super::*;
    use crate::execute::mint;
    use crate::msg::ExecuteMsg;
    use crate::query::admin;
    use crate::state::{Config, CONFIG};
    use cosmwasm_std::{
        entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    };

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let admin_addr: Option<Addr> = msg
            .admin
            .as_deref()
            .map(|s| deps.api.addr_validate(s))
            .transpose()?;

        let config = Config { admin: admin_addr };

        CONFIG.save(deps.storage, &config)?;

        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        };

        PassportContract::default().instantiate(
            deps.branch(),
            env,
            info,
            cw721_base_instantiate_msg,
        )?;

        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, cw721_base::ContractError> {
        // let config = CONFIG.load(deps.storage)?;
        // match config.admin {
        //     Some(admin) => {
        //         if admin == info.sender {
        //             PassportContract::default().execute(deps, env, info, msg)
        //         } else {
        //             Err(ContractError::Ownership(
        //                 cw721_base::OwnershipError::NotOwner,
        //             ))
        //         }
        //     }
        //     None => match msg {
        //         ExecuteMsg::Mint {} => mint(deps, env, info),
        //         _ => Err(ContractError::Ownership(
        //             cw721_base::OwnershipError::NotOwner,
        //         )),
        //     },
        // }
        match msg {
            ExecuteMsg::Mint {} => mint(deps, env, info),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Admin {} => to_binary(&admin(deps)?),
            _ => PassportContract::default().query(deps, env, msg.into()),
        }
    }
}
