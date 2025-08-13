//See https://docs.rs/osmosis-std/latest/osmosis_std/types/osmosis/twap/v1beta1/
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowRequest {
    #[prost(uint64, tag = "1")]
    pub pool_id: u64,

    #[prost(string, tag = "2")]
    pub base_asset: String,

    #[prost(string, tag = "3")]
    pub quote_asset: String,

    #[prost(message, optional, tag = "4")]
    pub start_time: Option<Timestamp>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArithmeticTwapToNowResponse {
    #[prost(string, tag = "1")]
    pub arithmetic_twap: String,
}
