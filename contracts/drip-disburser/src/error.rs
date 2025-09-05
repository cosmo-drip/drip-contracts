use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("Invalid withdrawal TTL: must be > 0 and <= `max_sec` \
    (got default_sec={default_sec}, max_sec={max_sec})")]
    InvalidTtl {
        default_sec: u64,
        max_sec: u64,
    },
    #[error("Invalid amount")]
    InvalidAmount {},
    #[error("Invalid payout duration: {reason}")]
    InvalidDuration { reason: String },
    #[error("Pending payout already exists; set replace_pending=true to overwrite")]
    PendingAlreadyExists {},
}
