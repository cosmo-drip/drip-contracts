use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Only supports channel with ibc version icq-1, got {version}")]
    InvalidIbcVersion { version: String },

    #[error("Invalid ibc packet, result should only contains 1 ResponseQuery")]
    InvalidResponseQuery,

    #[error("Failed to send interchain query")]
    InvalidResponseQueryCode,

    #[error("Twap data is empty")]
    EmptyTwap,

    #[error("Contract doesn't have an open IBC channel")]
    IbcChannelNotOpen,

    #[error("Contract already has an open IBC channel")]
    IbcChannelAlreadyOpen,

    #[error("Contract does not receive packets except for acknowledgements")]
    IbcReceiveNotAccepted,

    #[error("InvalidTwap: Invalid twap value received from the chain: {twap}. Should be a Decimal")]
    InvalidTwapString { twap: String },

    #[error("Unsupported expiration type")]
    UnsupportedExpirationType {},

    #[error("Unsupported pair request")]
    UnsupportedPairRequest {},

    #[error("Only ordered channels are supported")]
    OnlyOrderedChannel {},
}
