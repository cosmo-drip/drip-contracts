use crate::ack::AckWrapper;
use crate::state::{data_hash, ChannelInfo, RequestInfo, CHANNEL_INFO, REQ_BY_DATA};
use crate::ContractError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_json, to_json_binary, CosmosMsg, Decimal, DepsMut, Env, IbcBasicResponse, IbcChannel,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcOrder,
    IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, StdError,
    StdResult, WasmMsg,
};
use drip_disburser_interface::msg::ExecuteMsg::OnPayoutResponse;
use prost::Message;
use std::str::FromStr;
use crate::icq::{AcknowledgementResult, CosmosResponse, InterchainQueryPacketAck};
use crate::twap::ArithmeticTwapToNowResponse;

/// Version string for IBC compatibility
pub const IBC_VERSION: &str = "icq-1";

/// Handles the `OpenInit` and `OpenTry` parts of the IBC handshake.
///
/// Validates that no channel is already open and that the order and version are valid.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
    // Ensure no channel is already open
    if CHANNEL_INFO.may_load(deps.storage)?.is_some() {
        return Err(ContractError::IbcChannelAlreadyOpen);
    }

    validate_order_and_version(msg.channel(), msg.counterparty_version())?;
    Ok(None)
}

/// Handles the `OpenAck` and `OpenConfirm` parts of the IBC handshake.
///
/// Saves the channel information for future use after validating order and version.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // Ensure no channel is already open
    if CHANNEL_INFO.may_load(deps.storage)?.is_some() {
        return Err(ContractError::IbcChannelAlreadyOpen);
    }

    validate_order_and_version(msg.channel(), msg.counterparty_version())?;

    // Store the channel info
    let channel: IbcChannel = msg.into();
    let info = ChannelInfo {
        id: channel.endpoint.channel_id,
        counterparty_endpoint: channel.counterparty_endpoint,
        connection_id: channel.connection_id,
    };
    CHANNEL_INFO.save(deps.storage, &info)?;

    Ok(IbcBasicResponse::default())
}

/// Handles closing of an IBC channel.
///
/// Removes the channel information from storage.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let channel = msg.channel().endpoint.channel_id.clone();
    // Reset the state for the channel
    CHANNEL_INFO.remove(deps.storage);

    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_close")
        .add_attribute("channel", channel))
}

/// Handles incoming IBC packets.
///
/// This contract doesn't accept incoming packets, so it always returns an error.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    Err(ContractError::IbcReceiveNotAccepted)
}

/// Handles acknowledgments for previously sent IBC packets.
///
/// Processes TWAP query results and dispatches callbacks to requesters.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let key = data_hash(&msg.original_packet.data);
    let ibc_sequence = msg.original_packet.sequence;

    // Try to find the request data associated with this packet
    if let Some(request_list) = REQ_BY_DATA.may_load(deps.storage, key)? {
        let ack: AckWrapper = from_json(&msg.acknowledgement.data)?;

        match ack {
            AckWrapper::Result(_) => {
                // Process successful acknowledgment
                let ack_result: AcknowledgementResult = from_json(msg.acknowledgement.data)?;
                let packet_ack: InterchainQueryPacketAck = from_json(ack_result.result)?;

                // Process TWAP response and create callbacks
                let callbacks = process_twap_response(deps, packet_ack, request_list)?;

                Ok(IbcBasicResponse::new()
                    .add_messages(callbacks)
                    .add_attribute("method", "ibc_packet_ack")
                    .add_attribute("sequence", ibc_sequence.to_string()))
            },
            AckWrapper::Error(error) => {
                // Handle error acknowledgment
                Ok(IbcBasicResponse::new()
                    .add_attribute("method", "ibc_packet_ack")
                    .add_attribute("error", error.to_string())
                    .add_attribute("sequence", ibc_sequence.to_string()))
            }
        }
    } else {
        // Request not found in storage
        Ok(IbcBasicResponse::new()
            .add_attribute("method", "ibc_packet_ack")
            .add_attribute("error", "unknown request")
            .add_attribute("sequence", ibc_sequence.to_string()))
    }
}

/// Handles timeouts for IBC packets that weren't received by the counterparty.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_timeout"))
}

/// Validates IBC channel order and version compatibility.
///
/// Ensures that:
/// 1. The channel is unordered
/// 2. Both sides use compatible IBC versions
pub fn validate_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
    // We expect an unordered channel here. Ordered channels have the
    // property that if a message is lost the entire channel will stop
    // working until you start it again.
    if channel.order != IbcOrder::Unordered {
        return Err(ContractError::OnlyOrderedChannel {});
    }

    // Validate our version
    if channel.version != IBC_VERSION {
        return Err(ContractError::InvalidIbcVersion {
            version: channel.version.clone(),
        });
    }

    // Validate counterparty version if available
    if let Some(version) = counterparty_version {
        if version != IBC_VERSION {
            return Err(ContractError::InvalidIbcVersion {
                version: version.to_string(),
            });
        }
    }

    Ok(())
}

/// Processes a TWAP response and creates callback messages for requesters.
///
/// This is a higher-level function that orchestrates the extraction of TWAP price
/// and creation of callback messages.
fn process_twap_response(
    _deps: DepsMut,
    packet_ack: InterchainQueryPacketAck,
    request_info: Vec<RequestInfo>
) -> Result<Vec<CosmosMsg>, ContractError> {
    // Extract TWAP price from the packet acknowledgment
    let twap_price = extract_twap_price_from_ack(&packet_ack)?;

    // Create callback messages for each request
    create_callback_messages(request_info, twap_price)
}

/// Extracts the TWAP price from an interchain query packet acknowledgment.
///
/// Decodes the response data and validates it before extracting the price.
fn extract_twap_price_from_ack(packet_ack: &InterchainQueryPacketAck) -> Result<Decimal, ContractError> {
    let responses = decode_response(&packet_ack.data)?.responses;

    // Validate response structure
    if responses.len() != 1 {
        return Err(ContractError::InvalidResponseQuery);
    }

    let response = &responses[0];

    // Check for valid response code
    if response.code != 0 {
        return Err(ContractError::InvalidResponseQueryCode);
    }

    // Ensure response value is not empty
    if response.value.is_empty() {
        return Err(ContractError::EmptyTwap);
    }

    // Decode the TWAP response and parse price
    let twap_response: ArithmeticTwapToNowResponse = decode_twap_response(&response.value)?;
    let twap_price = Decimal::from_str(&twap_response.arithmetic_twap)
        .map_err(|_| ContractError::InvalidTwapString {
            twap: twap_response.arithmetic_twap.clone()
        })?;

    Ok(twap_price)
}

/// Creates callback messages for each request with the given TWAP price.
///
/// Each message invokes the OnPayoutResponse handler on the requester contract.
fn create_callback_messages(
    request_info: Vec<RequestInfo>,
    twap_price: Decimal
) -> Result<Vec<CosmosMsg>, ContractError> {
    request_info
        .into_iter()
        .map(|info| -> StdResult<_> {
            let exec_msg = OnPayoutResponse {
                price: twap_price,
                request_id: info.sequence,
            };

            let msg = CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: info.sender.to_string(),
                msg: to_json_binary(&exec_msg)?,
                funds: vec![],
            });

            Ok(msg)
        })
        .collect::<StdResult<Vec<_>>>()
        .map_err(ContractError::from)
}

/// Decodes a Cosmos query response from bytes.
pub fn decode_response(bytes: &[u8]) -> StdResult<CosmosResponse> {
    CosmosResponse::decode(bytes)
        .map_err(|err| StdError::generic_err(format!("fail to decode response query: {}", err)))
}

/// Decodes a TWAP response from bytes.
pub fn decode_twap_response(bytes: &[u8]) -> StdResult<ArithmeticTwapToNowResponse> {
    ArithmeticTwapToNowResponse::decode(bytes)
        .map_err(|err| StdError::generic_err(format!("fail to decode twap: {}", err)))
}
