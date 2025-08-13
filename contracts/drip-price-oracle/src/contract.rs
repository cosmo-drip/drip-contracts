#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{Reply, SubMsgResponse, SubMsgResult};
use cw2::set_contract_version;
use crate::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:drip-price-oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // todo:
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
        ExecuteMsg::UpdateAdmin { .. } => unimplemented!(),
        ExecuteMsg::RequestPrice { .. } => unimplemented!(),
        ExecuteMsg::OnPriceResponse { .. } => unimplemented!(),
        ExecuteMsg::TimeoutExpiredRequests { .. } => unimplemented!(),
        ExecuteMsg::AddAdapter { .. } => unimplemented!(),
        ExecuteMsg::RemoveAdapter { .. } => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    // match msg.id {
    //     REPLY_ID_PRICE_RESPONSE => handle_price_response(deps, env, msg),
    //     REPLY_ID_TIMEOUT => handle_timeout_response(deps, env, msg),
    //     _ => Err(ContractError::UnknownReplyId {}),
    // }

    match msg {
        Reply {
            id: MY_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgResponse { msg_responses, .. }),
            ..
        } if !msg_responses.is_empty() => {
            unimplemented!()
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {}
