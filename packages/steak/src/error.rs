use cosmwasm_std::{Coin, OverflowError, StdError, Uint128};
use cw_token::CwTokenError;
use thiserror::Error;

/// ## Description
/// This enum describes router-test contract errors!
#[derive(Error, Debug, PartialEq)]
pub enum SteakContractError {
    /// StdError
    #[error("{0}")]
    Std(#[from] StdError),

    /// CwTokenError
    ///
    #[error("{0}")]
    CwAsset(#[from] CwTokenError),

    /// Unauthorized Error
    #[error("Unauthorized")]
    Unauthorized {},

    /// Pair Info Not Found Error
    #[error("Pair Info not found, please add pair to adaptor to continue")]
    NotFound {},

    /// Invalid Pair Info Error
    #[error(
        "Invalid assets provided. Pool ID {pool_id} contains the following assets - {assets:?}"
    )]
    InvalidPairInfo {
        /// Provided pool ID
        pool_id: u64,
        /// Expected assets for given pool ID
        assets: Vec<Coin>,
    },

    /// Zero Withdrawalable Amount Error
    #[error("withdrawable amount is zero")]
    ZeroWithdrawableAmount {},

    /// Pair Info Not Found Error
    #[error("Invalid Message")]
    InvalidMessage {},

    /// Invalid Join Pool Assets Error
    #[error("Invalid number of assets provided to join pool. Must provide 1 or 2 assets.")]
    InvalidJoinPoolAssets {},

    /// Pair Info Not Found Error
    #[error("batch can only be submitted for unbonding after {est_unbond_start_time}")]
    InvalidSubmitBatch { est_unbond_start_time: u64 },

    /// Invalid Coin Sent Error
    #[error("Only the steak denom can be sent")]
    InvalidCoinSent {},

    #[error("Wrong amount sent. Expected {expected}, got {actual}")]
    WrongAmount {
        /// Expected amount
        expected: Uint128,
        /// Actual amount
        actual: Uint128,
    },

    /// No Coins Sent Error
    #[error("No coins sent")]
    NoCoinsSent {},

    /// Incorrect Queue Unbond Message Error
    #[error("Use of incorrect queue unbond message.")]
    IncorrectQueueUnbondMessage {},

    /// Invalid Callback Sender Error
    #[error("callbacks can only be invoked by the contract itself")]
    InvalidCallbackSender {},

    /// Invalid Reply ID Error
    #[error("invalid reply id: {}; must be 1, 2 or 3 {id}")]
    InvalidReplyId { id: u64 },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.31/thiserror/ for details.
}

impl From<OverflowError> for SteakContractError {
    fn from(o: OverflowError) -> Self {
        StdError::from(o).into()
    }
}
