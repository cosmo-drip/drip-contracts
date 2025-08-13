use crate::error::ContractError;
use crate::execute;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, TWAP_SETTINGS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:drip-price-adapter-twap-ibc";
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

    // assemble and store config
    let cfg = Config {
        admin,
    };
    CONFIG.save(deps.storage, &cfg)?;

    // Process each setting and save it to the map
    for setting in msg.twap_settings {
        TWAP_SETTINGS.save(
            deps.storage,
            (setting.base.clone(), setting.quote.clone()),
            &setting,
        )?;
    }

    // version for migrations
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RequestPrice {
            base,
            quote,
            sequence,
            valid_from,
            expiration,
        } => execute::request_price(deps, info.sender, base, quote, sequence, valid_from, expiration),
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
