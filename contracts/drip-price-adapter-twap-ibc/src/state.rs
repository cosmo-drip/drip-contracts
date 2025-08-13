use crate::msg::TwapSetting;
use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Deps, IbcEndpoint, StdResult};
use cw_storage_plus::{Item, Map};
use sha2::{Digest, Sha256};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const TWAP_SETTINGS: Map<(String, String), TwapSetting> = Map::new("twap_settings");

pub fn get_twap_setting(
    deps: Deps,
    base: String,
    quote: String,
) -> StdResult<Option<TwapSetting>> {
    TWAP_SETTINGS.may_load(deps.storage, (base, quote))
}

#[cw_serde]
pub struct ChannelInfo {
    /// id of this channel
    pub id: String,
    /// the remote channel/port we connect to
    pub counterparty_endpoint: IbcEndpoint,
    /// the connection this exists on (you can use to query client/consensus info)
    pub connection_id: String,
}

/// static info on one channel that doesn't change
pub const CHANNEL_INFO: Item<ChannelInfo> = Item::new("channel_info");

pub fn get_channel_id(deps: Deps) -> Result<String, ContractError> {
    match CHANNEL_INFO.may_load(deps.storage)? {
        Some(channel_info) => Ok(channel_info.id), // Return the item if it's loaded
        None => Err(ContractError::IbcChannelNotOpen),
    }
}

#[cw_serde]
pub struct RequestInfo {
    pub sender: Addr,
    pub sequence: u64,
}

/// key = sha256(packet.data), value = list of requesters (supports duplicates)
pub const REQ_BY_DATA: Map<[u8; 32], Vec<RequestInfo>> = Map::new("req_by_data");

pub fn data_hash(b: &Binary) -> [u8; 32] {
    Sha256::digest(b).into()
}
