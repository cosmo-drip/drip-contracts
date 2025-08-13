use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use cw_utils::Expiration;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub twap_settings: Vec<TwapSetting>,
}

#[cw_serde]
pub struct TwapSetting {
    pub pool_id: u64,
    pub base: String,
    pub quote: String,
    pub base_twap: String,
    pub quote_twap: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    RequestPrice {
        base: String,
        quote: String,
        sequence: u64,
        valid_from: Option<Expiration>,
        expiration: Expiration,
    },
    AddWhiteListedContract {
        contract_address: String,
    },
    RemoveWhitelistedContract {
        contract_address: String,
    },
    ModifyTwapSetting {
        twap_setting: TwapSetting,
    },
    ModifyAdmin {
        new_admin: String,
    },
}

#[cw_serde]
pub enum CallbackMsg {
    OnPriceResponse {
        request_id: u64,
        price: Decimal,
        price_timestamp: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
