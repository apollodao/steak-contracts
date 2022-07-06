use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw_asset::cw20_asset::{Cw20Asset, Cw20AssetInstantiator};
use steak::error::ContractError;
use steak::execute;
use steak::hub::{ExecuteMsg, MigrateMsg, QueryMsg};

pub type InstantiateMsg = steak::hub::InstantiateMsg<Cw20AssetInstantiator>;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    execute::instantiate(deps, env, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    execute::execute::<Cw20Asset>(deps, env, info, msg)
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    execute::reply::<Cw20AssetInstantiator>(deps, env, reply)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    execute::query(deps, env, msg)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
