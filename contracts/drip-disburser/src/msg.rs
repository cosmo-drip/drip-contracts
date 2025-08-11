use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128, Decimal256};
use cw_utils::Expiration;

#[cw_serde]
pub struct WithdrawalTtl {
    // todo: change it to expiration analog with relative duration in sec or blocks
    pub default_sec: u64,
    pub max_sec: u64,
    // todo: do we need to add Option<min_sec>?
}

#[cw_serde]
pub struct InstantiateMsg {
    pub settlement_asset_limit: Coin,
    pub quote_asset_limit: Coin,
    pub admin: Option<String>,
    pub recipient_addr: String,
    pub price_feeder_addr: String,
    pub payment_initiator_addrs: Vec<String>,
    pub funding_expiration: Expiration,
    pub withdrawal_ttl: WithdrawalTtl,
}

#[cw_serde]
pub enum ExecuteMsg {
    RequestPayout {
        amount_in_quote: Option<Uint128>,
        ttl_sec: Option<u64>, // todo: change it to expiration
        replace_pending: Option<bool>,
    },
    // todo: do we need to add OnTimeoutCallback?
    // todo: mb rename OnPriceCallback endpoint to OnPriceAsk?
    OnPriceCallback {
        // TODO: rethink type for price and is timestamp needed?
        price: Decimal256, // dec or num/den ?
        // price_timestamp: u64, // ?
        request_seq: u64,
    },
    Terminate {},
    CancelPendingPayout {
        expected_seq: Option<u64>,
    },
    UpdateAdmin { admin: Option<String> },
    AddPaymentInitiator { addr: String },
    RemovePaymentInitiator { addr: String },
    UpdateWithdrawalTtl { ttl: WithdrawalTtl },
    UpdatePriceFeeder { addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
