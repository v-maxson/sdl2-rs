use crate::{sys::*, error::SdlError, utils::get_sys_error};
use std::{sync::atomic::{AtomicBool, Ordering}, marker::PhantomData};

pub(crate) static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Represents an SDL2 context.
pub struct SdlContext(
    // PhantomData required to make this struct only initializable via `SdlContext::new`
    PhantomData<u8>
);

impl Drop for SdlContext {
    fn drop(&mut self) {
        INITIALIZED.store(false, Ordering::SeqCst);        

        unsafe {
            SDL_Quit();
        }
    }
}

impl SdlContext {
    /// Initializes SDL and returns a context to it.
    /// 
    /// ### Errors
    /// - [`SdlError::AlreadyInitialized`]
    /// - [`SdlError::SysError`]
    pub fn new() -> Result<Self, SdlError> {
        if INITIALIZED.load(Ordering::SeqCst) {
            return Err(SdlError::AlreadyInitialized)
        }

        if unsafe { SDL_Init(0) == 0 } {
            INITIALIZED.store(true, Ordering::SeqCst);

            Ok(Self(PhantomData::default()))
        } else {
            Err(SdlError::SysError(get_sys_error().unwrap()))
        }
    }
}
