use crate::{sys::*, error::SdlError, utils::get_sys_error, subsystems::{SdlSubsystem, markers::{SdlSubsystemMarker, Timer}, SdlSubsystemFlag}};
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

        #[cfg(feature = "log")]
        debug!("Calling 'SDL_Quit' via SdlContext drop.");
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
    #[doc(alias = "SDL_Init")]
    pub fn new() -> Result<Self, SdlError> {
        if INITIALIZED.load(Ordering::SeqCst) {
            return Err(SdlError::AlreadyInitialized(String::from("SDL2")))
        }

        #[cfg(feature = "log")] debug!("Calling 'SDL_Init(0)'");
        if unsafe { SDL_Init(0) == 0 } {
            INITIALIZED.store(true, Ordering::SeqCst);

            Ok(Self(PhantomData::default()))
        } else {
            Err(SdlError::SysError(get_sys_error().unwrap()))
        }
    }

    fn init<T: SdlSubsystemMarker>(&self, subsystem: SdlSubsystemFlag) -> Result<SdlSubsystem<T>, SdlError> {
        let initialized = subsystem.initialized_raw();

        if initialized.load(Ordering::SeqCst) {
            return Err(SdlError::AlreadyInitialized(format!("The {:?} subsystem", subsystem)));
        }

        #[cfg(feature = "log")] debug!("Calling 'SDL_QuitSubSystem({:?})'", subsystem);
        if unsafe { SDL_InitSubSystem(subsystem as _) == 0 } {
            initialized.store(true, Ordering::SeqCst);

            Ok(SdlSubsystem(Default::default(), SdlSubsystemFlag::Timer))
        } else {
            Err(SdlError::SysError(get_sys_error().unwrap()))
        }
    }

    /// Initializes the timer subsystem.
    #[doc(alias = "SDL_InitSubSystem(SDL_INIT_TIMER)")]
    #[inline]
    pub fn timer(&self) -> Result<SdlSubsystem<Timer>, SdlError> {
        self.init(SdlSubsystemFlag::Timer)
    }
}
