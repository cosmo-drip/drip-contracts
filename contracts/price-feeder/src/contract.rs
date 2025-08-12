#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:price-feeder";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // address normalization
    let admin = match &msg.admin {
        Some(a) => deps.api.addr_validate(a)?,
        None => info.sender.clone(),
    };
    let whitelisted_caller_addrs = msg
        .whitelisted_caller_addrs
        .into_iter()
        .map(|s| deps.api.addr_validate(&s))
        .collect::<Result<Vec<_>, _>>()?;

    // assemble and store config
    let cfg = Config {
        admin,
        twap_setting: msg.twap_setting,
        whitelisted_caller_addrs,
    };
    CONFIG.save(deps.storage, &cfg)?;

    // version for migrations
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::GetTokenPrice {
            base_asset_denom,
            quote_asset_denom,
        } => unimplemented!(),
        ExecuteMsg::AddWhiteListedContract {
            contract_address,
        } => unimplemented!(),
        ExecuteMsg::RemoveWhitelistedContract {
            contract_address,
        } => unimplemented!(),
        ExecuteMsg::ModifyTwapSetting {
            twap_setting,
        } => unimplemented!(),
        ExecuteMsg::ModifyAdmin {
            new_admin,
        } => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
