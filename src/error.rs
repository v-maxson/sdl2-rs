use std::ffi::NulError;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SdlError {
    /// SDL2 (or a subsystem) is already initialized.
    #[error("{0}")]
    AlreadyInitialized(String),

    /// An error occured within SDL2 itself.
    #[error("{0}")]
    SysError(String),

    /// See [`NulError`].
    #[error("{0}")]
    NulError(#[from] NulError)
}
