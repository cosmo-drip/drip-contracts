use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use cw_utils::Expiration;

#[cw_serde]
pub struct WithdrawalTtl {
    pub default_sec: u64,
    pub max_sec: u64,
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
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
