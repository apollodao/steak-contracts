use std::fmt;
use std::str::FromStr;

use cosmwasm_std::{Coin, CosmosMsg, StakingMsg, StdError, StdResult, Uint128};
use terra_cosmwasm::TerraMsgWrapper;

use crate::helpers::parse_coin;

//--------------------------------------------------------------------------------------------------
// Coins
//--------------------------------------------------------------------------------------------------

pub(crate) struct Coins(pub Vec<Coin>);

impl FromStr for Coins {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Ok(Self(vec![]));
        }

        Ok(Self(
            s.split(",")
                .filter(|coin_str| coin_str.len() > 0) // coin with zero amount may appeat as an empty string in the event log
                .collect::<Vec<&str>>()
                .iter()
                .map(|s| parse_coin(s))
                .collect::<StdResult<Vec<Coin>>>()?,
        ))
    }
}

impl fmt::Display for Coins {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coins_str = if self.0.len() == 0 {
            String::from("null")
        } else {
            self.0.iter().map(|coin| coin.to_string()).collect::<Vec<String>>().join(",")
        };
        write!(f, "{}", coins_str)
    }
}

impl Coins {
    pub fn add(mut self, coin_to_add: &Coin) -> StdResult<Self> {
        match self.0.iter_mut().find(|coin| coin.denom == coin_to_add.denom) {
            Some(coin) => {
                coin.amount = coin.amount.checked_add(coin_to_add.amount)?;
            },
            None => {
                self.0.push(coin_to_add.clone());
            },
        }
        Ok(self)
    }

    pub fn add_many(mut self, coins_to_add: &Coins) -> StdResult<Self> {
        for coin_to_add in &coins_to_add.0 {
            self = self.add(coin_to_add)?;
        }
        Ok(self)
    }
}

//--------------------------------------------------------------------------------------------------
// Delegation
//--------------------------------------------------------------------------------------------------

pub(crate) struct Delegation {
    pub validator: String,
    pub amount: Uint128,
}

impl Delegation {
    pub fn new<T: Into<Uint128>>(validator: &str, amount: T) -> Self {
        Self {
            validator: String::from(validator),
            amount: amount.into(),
        }
    }

    pub fn to_cosmos_msg(&self) -> CosmosMsg<TerraMsgWrapper> {
        CosmosMsg::Staking(StakingMsg::Delegate {
            validator: self.validator.clone(),
            amount: Coin::new(self.amount.u128(), "uluna"),
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Undelegation
//--------------------------------------------------------------------------------------------------

pub(crate) struct Undelegation {
    pub validator: String,
    pub amount: Uint128,
}

impl Undelegation {
    pub fn new<T: Into<Uint128>>(validator: &str, amount: T) -> Self {
        Self {
            validator: String::from(validator),
            amount: amount.into(),
        }
    }

    pub fn to_cosmos_msg(&self) -> CosmosMsg<TerraMsgWrapper> {
        CosmosMsg::Staking(StakingMsg::Undelegate {
            validator: self.validator.clone(),
            amount: Coin::new(self.amount.u128(), "uluna"),
        })
    }
}
