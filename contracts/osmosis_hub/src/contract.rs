use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw_token::implementations::osmosis::{OsmosisDenom, OsmosisDenomInstantiator};
use steak::error::SteakContractError;
use steak::execute;
use steak::hub::{ExecuteMsg, MigrateMsg, QueryMsg};

pub type InstantiateMsg = steak::hub::InstantiateMsg<OsmosisDenomInstantiator>;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, SteakContractError> {
    execute::instantiate::<OsmosisDenom, OsmosisDenomInstantiator>(deps, env, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, SteakContractError> {
    execute::execute::<OsmosisDenom>(deps, env, info, msg)
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, SteakContractError> {
    execute::reply::<OsmosisDenom, OsmosisDenomInstantiator>(deps, env, reply)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    execute::query::<OsmosisDenom>(deps, env, msg)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
