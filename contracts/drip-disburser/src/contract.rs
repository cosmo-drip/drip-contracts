#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, ensure, Uint128, Addr, StdError, WasmMsg, CosmosMsg, to_json_binary};
use cw2::set_contract_version;
use cw_utils::Expiration;
use drip_disburser_interface::msg::{Duration, DurationBounds, ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::error::ContractError;
use crate::state::{Config, PendingPayout, CONFIG, PENDING_PAYOUT};
use drip_price_oracle_interface::msg::ExecuteMsg as OracleExecuteMsg;

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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RequestPayout {
            amount_in_quote,
            duration_limit,
            replace_pending
        } => execute_request_payout(deps, env, info, amount_in_quote, duration_limit, replace_pending),
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

fn execute_request_payout(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount_in_quote: Option<Uint128>,
    duration: Option<Duration>,
    replace_pending: Option<bool>,
) -> Result<Response, ContractError> {
    // TODO: validate caller is allowed to request payout (payment initiator)
    // TODO: check contract lifecycle/state allows new payout requests (not terminated/paused)

    if let Some(a) = amount_in_quote {
        ensure!(a > Uint128::zero(), ContractError::InvalidAmount {});
    }

    let cfg = CONFIG.load(deps.storage)?;
    let expires_at = normalize_duration_to_expiration(&env, &cfg.payout_duration_bounds, duration)
        .map_err(|e| ContractError::InvalidDuration { reason: e.to_string() })?;

    let has_pending = PENDING_PAYOUT.may_load(deps.storage)?.is_some();
    if has_pending && !replace_pending.unwrap_or(false) {
        return Err(ContractError::PendingAlreadyExists {});
    }

    let amount = resolve_amount_in_quote(deps.as_ref(), &env, amount_in_quote)?;
    let pending = PendingPayout {
        amount_in_quote: amount,
        expires_at: expires_at.clone() // todo
    };

    PENDING_PAYOUT.save(deps.storage, &pending)?;

    let oracle_msg = OracleExecuteMsg::RequestPrice {
        base: cfg.settlement_asset_limit.denom.clone(), // todo
        quote: cfg.quote_asset_limit.denom.clone(),
        expiration: expires_at.clone(),
        valid_from: None,
        sequence: None,
    };
    let sub = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cfg.price_feeder_addr.to_string(),
        msg: to_json_binary(&oracle_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(sub)
        .add_attribute("action", "request_payout")
        .add_attribute(
            "amount_in_quote",
            amount_in_quote
                .map(|a| a.to_string())
                .unwrap_or_else(|| "ALL_AVAILABLE".into()),
        )
        // .add_attribute("expires_at", format_expiration(&env, &expires_at))
        // .add_attribute("replace_pending", replace_pending.to_string())
    )
}

fn resolve_amount_in_quote(
    deps: Deps,
    env: &Env,
    amount_in_quote: Option<Uint128>,
) -> StdResult<Uint128> {
    if let Some(a) = amount_in_quote {
        return Ok(a);
    }

    let cfg = CONFIG.load(deps.storage)?;
    // todo:

    // let bal: BalanceResponse = deps.querier.query(
    //     &QueryRequest::Bank(BankQuery::Balance {
    //         address: env.contract.address.to_string(),
    //         denom: cfg.quote_asset_limit.denom.clone(),
    //     })
    // )?;
    //
    // let mut amount = bal.amount.amount;
    //
    // let cap: Uint128 = cfg.quote_asset_limit.amount;
    // if amount > cap {
    //     amount = cap;
    // }
    //
    // Ok(amount)
    unimplemented!()
}

fn normalize_duration_to_expiration(
    env: &Env,
    bounds: &DurationBounds,
    duration: Option<Duration>,
) -> StdResult<Expiration> {
    let d = duration.unwrap_or_else(|| {
        // todo: do we need to use default blocks or seconds? mb prioritize to seconds?
        if bounds.default.blocks > 0 {
            Duration::Blocks(bounds.default.blocks)
        } else {
            Duration::Seconds(bounds.default.seconds)
        }
    });

    // todo: check StdError
    match d {
        Duration::Blocks(n) => {
            if let Some(min) = &bounds.min {
                ensure!(n >= min.blocks, StdError::generic_err("duration.blocks < min"));
            }
            if let Some(max) = &bounds.max {
                ensure!(n <= max.blocks, StdError::generic_err("duration.blocks > max"));
            }
            Ok(Expiration::AtHeight(env.block.height + n))
        }
        Duration::Seconds(s) => {
            if let Some(min) = &bounds.min {
                ensure!(s >= min.seconds, StdError::generic_err("duration.seconds < min"));
            }
            if let Some(max) = &bounds.max {
                ensure!(s <= max.seconds, StdError::generic_err("duration.seconds > max"));
            }
            Ok(Expiration::AtTime(env.block.time.plus_seconds(s)))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier},
        from_binary, CosmosMsg, WasmMsg, Uint128, Addr, MemoryStorage, OwnedDeps,
    };

    fn seed_config(deps: &mut OwnedDeps<MemoryStorage, MockApi, MockQuerier>) {
        use crate::state::{Config, CONFIG};
        use drip_disburser_interface::msg::{DurationLimit, DurationBounds};
        let cfg = Config {
            settlement_asset_limit: cosmwasm_std::coin(0, "SETTLE"),
            quote_asset_limit: cosmwasm_std::coin(0, "QUOTE"),
            admin: Addr::unchecked("admin"),
            recipient_addr: Addr::unchecked("recipient"),
            price_feeder_addr: Addr::unchecked("oracle"),
            payment_initiator_addrs: vec![],
            funding_expiration: Default::default(),
            payout_duration_bounds: DurationBounds {
                default: DurationLimit { blocks: 10, seconds: 30 },
                min: Some(DurationLimit { blocks: 5, seconds: 10 }),
                max: Some(DurationLimit { blocks: 60, seconds: 300 }),
            },
        };
        CONFIG.save(deps.as_mut().storage, &cfg).unwrap();
    }

    #[test]
    fn request_payout_happy_blocks() {
        let mut deps = mock_dependencies();
        let mut env = mock_env();
        seed_config(&mut deps);

        env.block.height = 100;

        let info = mock_info("caller", &[]);
        let amount = Some(Uint128::new(123));
        let duration = Some(Duration::Blocks(7));
        let replace = Some(false);

        let resp = execute_request_payout(
            deps.as_mut(),
            env.clone(),
            info,
            amount,
            duration,
            replace,
        ).expect("ok");

        assert_eq!(resp.messages.len(), 1);
        let msg = &resp.messages[0].msg;
        match msg {
            CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, msg, funds }) => {
                assert_eq!(contract_addr, "oracle");
                assert!(funds.is_empty());
                let decoded: OracleExecuteMsg = from_binary(msg).unwrap();
                match decoded {
                    OracleExecuteMsg::RequestPrice { base, quote, expiration, valid_from, sequence } => {
                        assert_eq!(base, "SETTLE");
                        assert_eq!(quote, "QUOTE");
                        assert_eq!(expiration, Expiration::AtHeight(107));
                        assert!(valid_from.is_none());
                        assert!(sequence.is_none());
                    }
                    _ => panic!("unexpected oracle msg"),
                }
            }
            _ => panic!("unexpected CosmosMsg"),
        }

        use crate::state::PENDING_PAYOUT;
        let pending = PENDING_PAYOUT.load(&deps.storage).expect("pending");
        assert_eq!(pending.amount_in_quote, Uint128::new(123));
        assert_eq!(pending.expires_at, Expiration::AtHeight(107));
    }

    #[test]
    fn request_payout_happy_seconds() {
        let mut deps = mock_dependencies();
        let mut env = mock_env();
        seed_config(&mut deps);

        env.block.time = env.block.time.plus_seconds(1000);
        let info = mock_info("caller", &[]);
        let resp = execute_request_payout(
            deps.as_mut(),
            env.clone(),
            info,
            Some(Uint128::new(1)),
            Some(Duration::Seconds(20)),
            None,
        ).unwrap();

        let msg = &resp.messages[0].msg;
        match msg {
            CosmosMsg::Wasm(WasmMsg::Execute { msg, .. }) => {
                let decoded: OracleExecuteMsg = from_binary(msg).unwrap();
                match decoded {
                    OracleExecuteMsg::RequestPrice { expiration, .. } => {
                        assert_eq!(expiration, Expiration::AtTime(env.block.time.plus_seconds(20)));
                    }
                    _ => panic!("unexpected oracle msg"),
                }
            }
            _ => panic!("unexpected CosmosMsg"),
        }
    }

    #[test]
    fn request_payout_reject_when_pending_and_no_replace() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        seed_config(&mut deps);

        PENDING_PAYOUT.save(
            deps.as_mut().storage,
            &PendingPayout {
                amount_in_quote: Uint128::new(5),
                expires_at: Expiration::AtHeight(999),
            },
        ).unwrap();

        let err = execute_request_payout(
            deps.as_mut(),
            env,
            mock_info("caller", &[]),
            Some(Uint128::new(7)),
            Some(Duration::Blocks(6)),
            Some(false), // replace_pending
        ).unwrap_err();

        match err {
            ContractError::PendingAlreadyExists {} => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn normalize_duration_respects_bounds() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        seed_config(&mut deps);

        let cfg = CONFIG.load(&deps.storage).unwrap();

        // lower than min
        let err = normalize_duration_to_expiration(&env, &cfg.payout_duration_bounds, Some(Duration::Blocks(4))).unwrap_err();
        assert!(err.to_string().contains("duration.blocks < min"));

        // bigger than max
        let err = normalize_duration_to_expiration(&env, &cfg.payout_duration_bounds, Some(Duration::Blocks(100))).unwrap_err();
        assert!(err.to_string().contains("duration.blocks > max"));

        // in bounds
        let ok = normalize_duration_to_expiration(&env, &cfg.payout_duration_bounds, Some(Duration::Blocks(10))).unwrap();
        assert_eq!(ok, Expiration::AtHeight(env.block.height + 10));
    }
}
