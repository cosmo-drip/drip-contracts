use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use cw_utils::Expiration;

#[cw_serde]
pub struct Config {}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct PriceKey {
    pub base: String,
    pub quote: String,
}

#[cw_serde]
pub struct Stamp {
    pub timestamp: u64,
    pub block_height: u64,
}

#[cw_serde]
pub enum StampOne {
    Timestamp(u64),
    BlockHeight(u64),
}

#[cw_serde]
pub struct PendingVal {
    pub price_key: PriceKey,
    pub expiration: StampOne,
    pub window_start: StampOne,
    pub inflight_request_timestamp: Stamp,
}

type Base = String;
type Quote = String;
type RequestTimestamp = u64;
type RequestBlockHeight = u64;
type InflightKey = (Base, Quote, RequestTimestamp, RequestBlockHeight);
type RequestId = u64; // based on inflight_sequence
type UserSequence = u64;

pub const INFLIGHT_SEQUENCE: Item<u64> = Item::new("inflight_sequence");

pub const INFLIGHT_REQUESTS: Map<InflightKey, RequestId> = Map::new("inflight_requests");

pub const PENDING_REQUESTS: Map<(RequestId, Addr, UserSequence), PendingVal> =
    Map::new("pending_requests");
