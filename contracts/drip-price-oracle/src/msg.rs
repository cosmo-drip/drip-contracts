use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal};
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub adapters: Option<Vec<Addr>>,
    pub admin: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    RequestPrice {
        base: String,
        quote: String,
        expiration: Expiration,
        /// Valid_from – the earliest point in time (or block height) from which the price is still acceptable.
        /// If `None`, only a fresh price will be requested from the adapter.
        /// If set, the oracle may serve a cached/batched price if it was obtained after this moment.
        valid_from: Option<Expiration>, // todo: Expiration or relative blocks/time?
        sequence: Option<u64>,
    },
    OnPriceResponse {
        request_id: u64,
        price: Decimal,
        price_timestamp: u64,
    },
    TimeoutExpiredRequests {
        batch_limit: u64
    },
    AddAdapter { adapter: Addr },
    RemoveAdapter { adapter: Addr },
    UpdateAdmin { admin: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
