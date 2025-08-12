use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub twap_setting: TwapSetting,
    pub whitelisted_caller_addrs: Vec<String>,
}

#[cw_serde]
pub struct TwapSetting {
    pub pool_id: u64,
    pub base_asset: String,
    pub quote_asset: String,
    pub base_twap_asset: String,
    pub quote_twap_asset: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    GetTokenPrice {
        base_asset_denom: String,
        quote_asset_denom: String,
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
pub enum ContractResponseMsg {
    ResponseTokenPrice {
        arithmetic_twap: String
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
