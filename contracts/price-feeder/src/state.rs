use crate::msg::{PriceFetchMode, TwapSetting};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub twap_setting: TwapSetting,
    pub price_fetch_mode: PriceFetchMode,
    pub whitelisted_caller_addrs: Vec<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");
