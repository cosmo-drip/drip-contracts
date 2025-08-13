use std::error::Error;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, to_json_binary, StdResult};

/// This is a generic ICS acknowledgement format.
/// Protobuf defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/core/channel/v1/channel.proto#L141-L147
/// This is compatible with the JSON serialization.
/// Wasmd uses this same wrapper for unhandled errors.
#[cw_serde]
pub enum AckWrapper {
    Result(Binary),
    Error(String),
}

// create a serialized success message
pub fn ack_success<T: serde::Serialize>(data: &T) -> StdResult<Binary> {
    let res = AckWrapper::Result(to_json_binary(data)?);
    to_json_binary(&res)
}

// create a serialized error message
pub fn ack_fail<E: Error>(err: E) -> StdResult<Binary> {
    let res = AckWrapper::Error(err.to_string());
    to_json_binary(&res)
}
