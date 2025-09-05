use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
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

#[cw_serde]
pub struct PendingPayout {
    // pub requester: Addr,
    // pub amount_in_quote: Option<Coin>,
    pub amount_in_quote: Uint128,
    pub expires_at: Expiration,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PENDING_PAYOUT: Item<PendingPayout> = Item::new("pending_payout");
