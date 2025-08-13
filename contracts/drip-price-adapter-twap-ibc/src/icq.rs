/// InterchainQueryPacketData is comprised of raw query.
// See https://docs.rs/crate/terra-proto-rs/latest/source/models/icq.v1.rs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
)]
pub struct InterchainQueryPacketData {
    #[prost(bytes = "vec", tag = "1")]
    pub data: Vec<u8>,
    /// optional memo
    #[prost(string, tag = "2")]
    pub memo: String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
)]
pub struct AcknowledgementResult {
    #[prost(bytes = "vec", tag = "1")]
    pub result: Vec<u8>,
}

/// InterchainQueryPacketAck is comprised of an ABCI query response with non-deterministic fields left empty (e.g. Codespace, Log, Info and ...).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
)]
pub struct InterchainQueryPacketAck {
    #[prost(bytes = "vec", tag = "1")]
    pub data: Vec<u8>,
}

/// CosmosQuery contains a list of tendermint ABCI query requests. It should be used when sending queries to an SDK host chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosQuery {
    #[prost(message, repeated, tag = "1")]
    pub requests: Vec<AbciQueryRequest>,
}

/// CosmosResponse contains a list of tendermint ABCI query responses. It should be used when receiving responses from an SDK host chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: Vec<AbciQueryResponse>,
}

//See https://docs.rs/cosmos-sdk-proto/latest/cosmos_sdk_proto/cosmos/base/tendermint/v1beta1/index.html
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciQueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: Vec<u8>,
    #[prost(string, tag = "2")]
    pub path: String,
    #[prost(int64, tag = "3")]
    pub height: i64,
    #[prost(bool, tag = "4")]
    pub prove: bool,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
)]
pub struct AbciQueryResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,

    #[prost(int64, tag = "2")]
    pub index: i64,

    #[prost(bytes = "vec", tag = "3")]
    pub key: Vec<u8>,

    #[prost(bytes = "vec", tag = "4")]
    pub value: Vec<u8>,

    #[prost(int64, tag = "5")]
    pub height: i64,
}
