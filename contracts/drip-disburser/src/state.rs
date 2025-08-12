use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use cw_utils::Expiration;
use drip_disburser_interface::msg::DurationBounds;

#[cw_serde]
pub struct Config {
    pub settlement_asset_limit: Coin,
    pub quote_asset_limit: Coin,
    pub admin: Addr,
    pub recipient_addr: Addr,
    pub price_feeder_addr: Addr,
    pub payment_initiator_addrs: Vec<Addr>,
    pub funding_expiration: Expiration,
    pub payout_duration_bounds: DurationBounds,
}

pub const CONFIG: Item<Config> = Item::new("config");
