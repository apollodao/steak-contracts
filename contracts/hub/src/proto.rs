/// MsgCreateDenom is the sdk.Msg type for allowing an account to create
/// a new denom. It requires a sender address and a subdenomination.
/// The (sender_address, sub_denomination) pair must be unique and cannot be
/// re-used. The resulting denom created is `factory/{creator
/// address}/{subdenom}`. The resultant denom's admin is originally set to be the
/// creator, but this can be changed later. The token denom does not indicate the
/// current admin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDenom {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag="2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// MsgCreateDenomResponse is the return value of MsgCreateDenom
/// It returns the full string of the newly created denom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDenomResponse {
    #[prost(string, tag="1")]
    pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint is the sdk.Msg type for allowing an admin account to mint
/// more of a token.  For now, we only support minting to the sender account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMint {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintResponse {
}
/// MsgBurn is the sdk.Msg type for allowing an admin account to burn
/// a token.  For now, we only support burning from the sender account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnResponse {
}
// // ===================== MsgForceTransfer
// message MsgForceTransfer {
//   string sender = 1 [ (gogoproto.moretags) = "yaml:\"sender\"" ];
//   cosmos.base.v1beta1.Coin amount = 2 [
//     (gogoproto.moretags) = "yaml:\"amount\"",
//     (gogoproto.nullable) = false
//   ];
//   string transferFromAddress = 3
//       [ (gogoproto.moretags) = "yaml:\"transfer_from_address\"" ];
//   string transferToAddress = 4
//       [ (gogoproto.moretags) = "yaml:\"transfer_to_address\"" ];
// }

// message MsgForceTransferResponse {}

/// MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign
/// adminship of a denom to a new account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChangeAdmin {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_admin: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChangeAdminResponse {
}
