use thiserror::Error;

/// Error type for machine ID operations.
#[derive(Error, Debug)]
pub enum MachineIdError {
    #[error("Failed to get machine ID: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Machine ID not found")]
    NotFound,

    #[error("Platform not supported")]
    UnsupportedPlatform,
}
