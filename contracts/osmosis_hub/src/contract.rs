use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw_asset::osmosis::{OsmosisCoin, OsmosisDenomInstantiator};
use steak::error::SteakContractError;
use steak::execute;
use steak::hub::{ExecuteMsg, MigrateMsg, QueryMsg};
use steak::state::SteakToken;

pub type InstantiateMsg = steak::hub::InstantiateMsg<OsmosisDenomInstantiator>;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, SteakContractError> {
    execute::instantiate(deps, env, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, SteakContractError> {
    execute::execute::<OsmosisCoin>(deps, env, info, msg)
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, SteakContractError> {
    execute::reply::<OsmosisDenomInstantiator>(deps, env, reply)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    execute::query(deps, env, msg)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
