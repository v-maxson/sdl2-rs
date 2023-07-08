use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdlError {
    /// SDL2 (or a subsystem) is already initialized.
    #[error("{0} is already initialized.")]
    AlreadyInitialized(String),

    /// An error occured within SDL2 itself.
    #[error("SDL2 Error: {0}")]
    SysError(String)
}
