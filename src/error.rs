use thiserror::Error;

#[derive(Debug, Error)]
pub enum SdlError {
    /// SDL2 is already initialized.
    #[error("SDL2 is already initialized.")]
    AlreadyInitialized,

    /// SDL2 is not initialized.
    #[error("SDL2 is not initialized.")]
    NotInitialized,

    /// An error occured within SDL2 itself.
    #[error("SDL2 Error: {0}")]
    SysError(String)
}
