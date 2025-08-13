use crate::icq::{AbciQueryRequest, CosmosQuery, InterchainQueryPacketData};
use crate::msg::TwapSetting;
use crate::state::{data_hash, get_channel_id, get_twap_setting, RequestInfo, REQ_BY_DATA};
use crate::twap::{ArithmeticTwapToNowRequest, Timestamp};
use crate::ContractError;
use cosmwasm_std::{to_json_binary, Addr, DepsMut, IbcMsg, IbcTimeout, Response, StdResult};
use cw_utils::Expiration;
use prost::Message;

pub const OSMOSIS_QUERY_TWAP_PATH: &str = "/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow";
pub const ICQ_TWAP_MEMO: &str = "TWAP ICQ request";

/// Request price information via an interchain query for a specific trading pair.
///
/// # Arguments
///
/// * `deps` - Dependencies providing access to storage, API, and querier
/// * `sender` - The address making the request
/// * `base` - Base asset of the trading pair
/// * `quote` - Quote asset of the trading pair
/// * `sequence` - Sequence number for the request
/// * `valid_from` - Optional time from which the TWAP calculation should start
/// * `expiration` - When the request should expire
///
/// # Returns
///
/// * `Response` - Success response with appropriate attributes and messages
/// * `ContractError` - Error if the request fails
pub fn request_price(
    deps: DepsMut,
    sender: Addr,
    base: String,
    quote: String,
    sequence: u64,
    valid_from: Option<Expiration>,
    expiration: Expiration,
) -> Result<Response, ContractError> {
    // Get TWAP setting for the requested pair
    let twap_setting = get_twap_setting(deps.as_ref(), base, quote)?
        .ok_or(ContractError::UnsupportedPairRequest {})?;

    // Construct the ICQ TWAP request and prepare data for storage
    let packet_data = construct_icq_twap_request(twap_setting, valid_from)?;
    let payload = to_json_binary(&packet_data)?;
    let key = data_hash(&payload);

    // Check if this is an existing request
    let request_exists = REQ_BY_DATA.has(deps.storage, key);
    let request_info = RequestInfo {
        sender: sender.clone(),
        sequence,
    };

    if request_exists {
        // Add sender to existing request (avoiding duplicates)
        REQ_BY_DATA.update(deps.storage, key, |maybe_list| -> StdResult<_> {
            let mut list = maybe_list.unwrap_or_default();
            list.push(request_info);
            Ok(list)
        })?;

        Ok(Response::new()
            .add_attribute("method", "joined_existing_twap_icq_query"))
    } else {
        // Create a new request with this sender
        REQ_BY_DATA.save(deps.storage, key, &vec![request_info])?;

        let channel_id = get_channel_id(deps.as_ref())?;

        // Currently only supports AtTime expiration
        let timeout = match expiration {
            Expiration::AtTime(ts) => IbcTimeout::with_timestamp(ts),
            _ => return Err(ContractError::UnsupportedExpirationType {}),
        };

        // Create and send IBC message
        let ibc_msg = IbcMsg::SendPacket {
            channel_id: channel_id.clone(),
            data: payload,
            timeout,
        };

        Ok(Response::new()
            .add_attribute("method", "send_twap_icq_query")
            .add_attribute("channel", channel_id)
            .add_message(ibc_msg))
    }
}

/// Constructs a TWAP request packet for interchain querying
///
/// # Arguments
///
/// * `twap_setting` - Configuration for the TWAP calculation
/// * `valid_from` - Optional timestamp from which to start the TWAP calculation
///
/// # Returns
///
/// * `Result<InterchainQueryPacketData, ContractError>` - The constructed packet or an error
fn construct_icq_twap_request(
    twap_setting: TwapSetting,
    valid_from: Option<Expiration>,
) -> Result<InterchainQueryPacketData, ContractError> {
    // Convert CosmWasm timestamp to Protobuf timestamp if provided
    let start_time = valid_from
        .and_then(|exp| match exp {
            Expiration::AtTime(ts) => Some(Timestamp {
                seconds: ts.seconds() as i64,
                nanos: ts.nanos() as i32,
            }),
            _ => None,
        });

    // Create the TWAP request
    let twap_request = ArithmeticTwapToNowRequest {
        pool_id: twap_setting.pool_id,
        base_asset: twap_setting.base_twap,
        quote_asset: twap_setting.quote_twap,
        start_time,
    };

    // Create and encode the ABCI query
    let abci_query = AbciQueryRequest {
        data: twap_request.encode_to_vec(),
        path: OSMOSIS_QUERY_TWAP_PATH.to_string(),
        height: 0,
        prove: false,
    };

    // Create the Cosmos query with the ABCI request
    let cosmos_query = CosmosQuery {
        requests: vec![abci_query]
    };

    // Create the final interchain query packet
    let packet_data = InterchainQueryPacketData {
        data: cosmos_query.encode_to_vec(),
        memo: ICQ_TWAP_MEMO.to_string(),
    };

    Ok(packet_data)
}
