use cosmwasm_std::{
    entry_point, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw_token::cw20::Cw20;
use steak::error::SteakContractError;
use steak::execute;
use steak::helpers::merge_responses;
use steak::hub::{ExecuteMsg, MigrateMsg, QueryMsg};
use steak::state::ItemStorage;

pub type InstantiateMsg = steak::hub::InstantiateMsg;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, SteakContractError> {
    let cw20 = Cw20(Addr::unchecked(String::default()));
    execute::instantiate(deps, env, msg, cw20)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, SteakContractError> {
    execute::execute::<Cw20>(deps, env, info, msg)
}

#[entry_point]
pub fn reply(mut deps: DepsMut, env: Env, reply: Reply) -> Result<Response, SteakContractError> {
    let save_token_response = Cw20::save_token(deps.branch(), &env, &reply, &Cw20::get_item())?;

    let reply_response = execute::reply::<Cw20>(deps, env, reply)?;

    Ok(merge_responses(vec![save_token_response, reply_response]))
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    execute::query::<Cw20>(deps, env, msg)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
