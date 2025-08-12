use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128, Decimal};
use cw_utils::Expiration;

#[cw_serde]
pub enum Duration {
    Blocks(u64),
    Seconds(u64),
    // NoExpiry {},
}

#[cw_serde]
pub struct DurationLimit {
    blocks: u64,
    seconds: u64,
}

#[cw_serde]
pub struct DurationBounds {
    pub default: DurationLimit,
    pub max: Option<DurationLimit>,
    pub min: Option<DurationLimit>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub settlement_asset_limit: Coin,
    pub quote_asset_limit: Coin,
    pub admin: Option<String>,
    pub recipient_addr: String,
    pub oracle_addr: String,
    pub payment_initiator_addrs: Vec<String>,
    pub funding_expiration: Expiration,
    pub payout_duration_bounds: DurationBounds,
}

#[cw_serde]
pub enum ExecuteMsg {
    RequestPayout {
        amount_in_quote: Option<Uint128>,
        duration_limit: Option<Duration>,
        replace_pending: Option<bool>,
    },
    OnPayoutResponse {
        price: Decimal,
        request_id: u64,
    },
    OnPayoutTimeout {
        request_id: u64,
    },
    Terminate {},
    CancelPendingPayout {
        expected_seq: Option<u64>,
    },
    UpdateAdmin { admin: Option<String> },
    AddPaymentInitiator { addr: String },
    RemovePaymentInitiator { addr: String },
    UpdateWithdrawalTtl { ttl: DurationBounds },
    UpdatePriceFeeder { addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
