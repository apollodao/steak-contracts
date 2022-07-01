use std::vec;

use apollo_proto_rust::{
    cosmos::base::v1beta1::Coin as ProtoCoin,
    osmosis::tokenfactory::v1beta1::{MsgBurn, MsgCreateDenom, MsgMint},
};
use apollo_protocol::utils::parse_contract_addr_from_instantiate_event;
use cosmwasm_std::{
    to_binary, Addr, BalanceResponse, BankMsg, BankQuery, Binary, Coin, CosmosMsg, DepsMut, Env,
    MessageInfo, QuerierWrapper, Reply, Response, StdError, StdResult, SubMsg, SubMsgResponse,
    Uint128, WasmMsg, WasmQuery,
};
use cw20_base::msg::{ExecuteMsg as Cw20ExecuteMsg, QueryMsg as Cw20QueryMsg};
use cw_storage_plus::Item;
use prost::Message;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw20::BalanceResponse as Cw20BalanceResponse;
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;

pub const REPLY_REGISTER_RECEIVED_COINS: u64 = 1;
pub const REPLY_SAVE_OSMOSIS_DENOM: u64 = 2;
pub const REPLY_SAVE_CW20_ADDRESS: u64 = 3;

/// Unwrap a `Reply` object to extract the response
/// TODO: Copied from larrys steakhouse. Move to protocol
pub(crate) fn unwrap_reply(reply: Reply) -> StdResult<SubMsgResponse> {
    reply.result.into_result().map_err(StdError::generic_err)
}

pub fn save_cw20_address(
    deps: DepsMut,
    res: SubMsgResponse,
    item: Item<Token>,
) -> StdResult<Response> {
    let address = parse_contract_addr_from_instantiate_event(deps.as_ref(), res.events)
        .map_err(|e| StdError::generic_err(format!("{}", e)))?;

    item.save(deps.storage, &Token::Cw20 { address })?;

    Ok(Response::default())
}

fn parse_osmosis_denom_from_event(response: SubMsgResponse) -> StdResult<String> {
    let event = response
        .events
        .iter()
        .find(|event| event.ty == "create_denom")
        .ok_or_else(|| StdError::generic_err("cannot find `create_denom` event"))?;

    let denom = &event
        .attributes
        .iter()
        .find(|attr| attr.key == "new_token_denom")
        .ok_or_else(|| StdError::generic_err("cannot find `new_token_denom` attribute"))?
        .value;

    Ok(denom.to_string())
}

pub fn save_osmosis_denom(
    deps: DepsMut,
    res: SubMsgResponse,
    item: Item<Token>,
) -> StdResult<Response> {
    let denom = parse_osmosis_denom_from_event(res)?;

    item.save(deps.storage, &Token::Osmosis { denom })?;

    Ok(Response::default())
}

pub fn reply_save_token(deps: DepsMut, reply: Reply) -> StdResult<Response> {
    let res = unwrap_reply(reply.clone())?;
    let token_item_key = TOKEN_ITEM_KEY.load(deps.storage)?;
    let item: Item<Token> = Item::new(&token_item_key);
    match reply.id {
        REPLY_SAVE_OSMOSIS_DENOM => save_osmosis_denom(deps, res, item),
        REPLY_SAVE_CW20_ADDRESS => save_cw20_address(deps, res, item),
        id => Err(StdError::generic_err(format!(
            "invalid reply id: {}; must be 2-3",
            id
        ))),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TokenInitInfo {
    Osmosis {
        subdenom: String,
    },
    Cw20 {
        label: String,
        admin: Option<String>,
        code_id: u64,
        cw20_init_msg: Box<Cw20InstantiateMsg>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInstantiator {
    pub item_key: String,
    pub init_info: TokenInitInfo,
}

const TOKEN_ITEM_KEY: Item<String> = Item::new("token_item_key");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct OsmosisCreateDenomMsg {
    sender: String,
    subdenom: String,
}

impl TokenInstantiator {
    pub fn instantiate(&self, deps: DepsMut, env: Env) -> StdResult<SubMsg> {
        TOKEN_ITEM_KEY.save(deps.storage, &self.item_key)?;

        match self.init_info.clone() {
            TokenInitInfo::Osmosis { subdenom } => {
                let msg = MsgCreateDenom {
                    subdenom,
                    sender: env.contract.address.to_string(),
                };
                let msg_bin = Binary::from(msg.encode_to_vec());
                Ok(SubMsg::reply_always(
                    CosmosMsg::Stargate {
                        type_url: "/osmosis.tokenfactory.v1beta1.MsgCreateDenom".to_string(),
                        value: msg_bin,
                    },
                    REPLY_SAVE_OSMOSIS_DENOM,
                ))
            }
            TokenInitInfo::Cw20 {
                cw20_init_msg,
                label,
                admin,
                code_id,
            } => Ok(SubMsg::reply_always(
                WasmMsg::Instantiate {
                    admin,
                    code_id,
                    msg: to_binary(&cw20_init_msg)?,
                    funds: vec![],
                    label,
                },
                REPLY_SAVE_CW20_ADDRESS,
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Token {
    Osmosis { denom: String },
    Cw20 { address: Addr },
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Osmosis { denom } => denom.to_owned(),
            Token::Cw20 { address } => address.to_string(),
        }
    }
}

/// Find the amount of a denom sent along a message, assert it is non-zero, and no other denom were
/// sent together
/// TODO: Took from steakcontracts. Move out to protocol utils and use here and in main steak contracts
pub(crate) fn parse_received_fund(funds: &[Coin], denom: &str) -> StdResult<Uint128> {
    if funds.len() != 1 {
        return Err(StdError::generic_err(format!(
            "must deposit exactly one coin; received {}",
            funds.len()
        )));
    }

    let fund = &funds[0];
    if fund.denom != denom {
        return Err(StdError::generic_err(format!(
            "expected {} deposit, received {}",
            denom, fund.denom
        )));
    }

    if fund.amount.is_zero() {
        return Err(StdError::generic_err("deposit amount must be non-zero"));
    }

    Ok(fund.amount)
}

impl Token {
    pub fn mint(&self, env: &Env, amount: Uint128, recipient: String) -> StdResult<CosmosMsg> {
        match self {
            Token::Osmosis { denom } => {
                let msg = MsgMint {
                    amount: Some(ProtoCoin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                    sender: env.contract.address.to_string(),
                };

                let msg_bin = Binary::from(msg.encode_to_vec());

                Ok(CosmosMsg::Stargate {
                    type_url: "/osmosis.tokenfactory.v1beta1.MsgMint".to_string(),
                    value: msg_bin,
                })
            }
            Token::Cw20 { address } => Ok(WasmMsg::Execute {
                contract_addr: address.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Mint { amount, recipient })?,
                funds: vec![],
            }
            .into()),
        }
    }

    pub fn burn(&self, env: &Env, amount: Uint128) -> StdResult<CosmosMsg> {
        match self {
            Token::Osmosis { denom } => {
                let msg = MsgBurn {
                    amount: Some(ProtoCoin {
                        denom: denom.to_string(),
                        amount: amount.to_string(),
                    }),
                    sender: env.contract.address.to_string(),
                };

                let msg_bin = Binary::from(msg.encode_to_vec());
                Ok(CosmosMsg::Stargate {
                    type_url: "/osmosis.tokenfactory.v1beta1.MsgBurn".to_string(),
                    value: msg_bin,
                })
            }
            Token::Cw20 { address } => Ok(WasmMsg::Execute {
                contract_addr: address.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Burn { amount })?,
                funds: vec![],
            }
            .into()),
        }
    }

    pub fn transfer(&self, _env: &Env, amount: Uint128, recipient: String) -> StdResult<CosmosMsg> {
        match self {
            Token::Osmosis { denom } => Ok(BankMsg::Send {
                to_address: recipient,
                amount: vec![Coin {
                    amount,
                    denom: denom.to_string(),
                }],
            }
            .into()),
            Token::Cw20 { address } => Ok(WasmMsg::Execute {
                contract_addr: address.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer { amount, recipient })?,
                funds: vec![],
            }
            .into()),
        }
    }

    /// Assert that `amount` of token has been received. For Osmosis this function asserts that the
    /// correct amount was sent along the message and returns None. For CW20 this function returns
    /// Some([`CosmosMsg::Wasm`]) containing [`WasmMsg::Execute`] to execute
    /// [`Cw20ExecuteMsg::TransferFrom`] `amount` of tokens from the `info.sender` to
    /// `env.contract.address`.
    ///
    /// # Arguments
    /// * `env` - The environment of the contract
    /// * `info` - The message info
    /// * `amount` - The amount of tokens received
    ///
    /// # Returns
    /// * `None` - If Osmosis
    /// * `Some([`CosmosMsg::Wasm`])` - If CW20
    pub fn assert_received_token(
        &self,
        env: &Env,
        info: &MessageInfo,
        amount: Uint128,
    ) -> StdResult<Option<CosmosMsg>> {
        match self {
            Token::Osmosis { denom } => {
                let received_amount = parse_received_fund(&info.funds, denom)?;
                if received_amount != amount {
                    return Err(StdError::generic_err("amount differs from received amount"));
                }
                Ok(None)
            }
            Token::Cw20 { address } => {
                let transfer_from_msg = WasmMsg::Execute {
                    contract_addr: address.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                        owner: info.sender.to_string(),
                        recipient: env.contract.address.to_string(),
                        amount,
                    })?,
                    funds: vec![],
                };
                Ok(Some(transfer_from_msg.into()))
            }
        }
    }

    pub fn query_balance(&self, querier: &QuerierWrapper, address: Addr) -> StdResult<Uint128> {
        match self {
            Token::Osmosis { denom } => {
                let query = BankQuery::Balance {
                    address: address.to_string(),
                    denom: denom.to_string(),
                };
                let res: BalanceResponse = querier.query(&query.into())?;
                Ok(res.amount.amount)
            }
            Token::Cw20 { address } => {
                let query = WasmQuery::Smart {
                    contract_addr: address.to_string(),
                    msg: to_binary(&Cw20QueryMsg::Balance {
                        address: address.to_string(),
                    })?,
                };
                let res: Cw20BalanceResponse = querier.query(&query.into())?;
                Ok(res.balance)
            }
        }
    }
}
