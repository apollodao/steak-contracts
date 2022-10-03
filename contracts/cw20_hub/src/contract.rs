use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw_token::cw20::Cw20;
use cw_token::{CwTokenError, Instantiate};
use steak::error::SteakContractError;
use steak::execute;
use steak::hub::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, SteakContractError> {
    execute::instantiate::<Cw20>(deps, env, msg)
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
    match Cw20::reply_save_token(deps.branch(), &env, &reply) {
        Ok(res) => Ok(res),
        Err(e) => {
            match e {
                // continue to default reply id match arm if error is InvalidReplyId
                CwTokenError::InvalidReplyId {} => execute::base_reply(deps, env, reply),
                _ => Err(e.into()),
            }
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, SteakContractError> {
    execute::query::<Cw20>(deps, env, msg)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
