#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, ensure
};
use cw2::set_contract_version;


use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:drip-disburser";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // todo: add more checks & parameter validations

    // address normalization
    let admin = match &msg.admin {
        Some(a) => deps.api.addr_validate(a)?,
        None => info.sender.clone(),
    };
    let recipient = deps.api.addr_validate(&msg.recipient_addr)?;
    let oracle = deps.api.addr_validate(&msg.oracle_addr)?;
    let initiators = msg
        .payment_initiator_addrs
        .into_iter()
        .map(|s| deps.api.addr_validate(&s))
        .collect::<Result<Vec<_>, _>>()?;

    // assemble and store config
    let cfg = Config {
        settlement_asset_limit: msg.settlement_asset_limit,
        quote_asset_limit: msg.quote_asset_limit,
        admin,
        recipient_addr: recipient,
        price_feeder_addr: oracle,
        payment_initiator_addrs: initiators,
        funding_expiration: msg.funding_expiration,
        payout_duration_bounds: msg.payout_duration_bounds,
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
        ExecuteMsg::RequestPayout {
            amount_in_quote,
            duration_limit: ttl_sec,
            replace_pending
        } => unimplemented!(),
        ExecuteMsg::OnPayoutResponse {
            price,
            // price_timestamp,
            request_id: request_seq
        } => unimplemented!(),
        ExecuteMsg::Terminate {} => unimplemented!(),
        ExecuteMsg::CancelPendingPayout {
            expected_seq
        } => unimplemented!(),
        ExecuteMsg::UpdateAdmin {
            admin
        } => unimplemented!(),
        ExecuteMsg::AddPaymentInitiator {
            addr
        } => unimplemented!(),
        ExecuteMsg::RemovePaymentInitiator {
            addr
        } => unimplemented!(),
        ExecuteMsg::UpdatePriceFeeder {
            addr
        } => unimplemented!(),
        ExecuteMsg::UpdateWithdrawalTtl {
            ttl
        } => unimplemented!(),
        ExecuteMsg::OnPayoutTimeout {
            request_id
        } => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
