use crate::{sys::*, error::SdlError, utils::get_sys_error, subsystems::{SdlSubsystem, markers::{SdlSubsystemMarker, Timer, Audio}, SdlSubsystemFlag, AUDIO_INITIALIZED}};
use std::{sync::atomic::{AtomicBool, Ordering}, marker::PhantomData, ffi::CString};

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
            return Err(SdlError::AlreadyInitialized(String::from("SDL2 is already initialized.")))
        }

        #[cfg(feature = "log")] debug!("Calling 'SDL_Init'");
        if unsafe { SDL_Init(0) == 0 } {
            INITIALIZED.store(true, Ordering::SeqCst);

            Ok(Self(PhantomData::default()))
        } else {
            Err(SdlError::SysError(get_sys_error().unwrap()))
        }
    }

    fn init_subsystem<T: SdlSubsystemMarker>(&self, subsystem: SdlSubsystemFlag) -> Result<SdlSubsystem<T>, SdlError> {
        let initialized = subsystem.initialized_raw();

        if initialized.load(Ordering::SeqCst) {
            return Err(SdlError::AlreadyInitialized(format!("The {:?} subsystem is already initialized.", subsystem)));
        }

        #[cfg(feature = "log")] debug!("Calling 'SDL_InitSubSystem({:?})'", subsystem);
        if unsafe { SDL_InitSubSystem(subsystem as _) == 0 } {
            initialized.store(true, Ordering::SeqCst);

            Ok(SdlSubsystem {
                t: Default::default(),
                subsystem,
                quitter: Option::None
            })
        } else {
            Err(SdlError::SysError(get_sys_error().unwrap()))
        }
    }

    /// Initializes the timer subsystem.
    /// 
    /// ### Errors
    /// - [`SdlError::AlreadyInitialized`]
    /// - [`SdlError::SysError`]
    #[doc(alias = "SDL_InitSubSystem(SDL_INIT_TIMER)")]
    #[inline]
    pub fn timer(&self) -> Result<SdlSubsystem<Timer>, SdlError> {
        self.init_subsystem(SdlSubsystemFlag::Timer)
    }

    /// Initializes the audio subsystem.
    /// 
    /// ### Errors
    /// - [`SdlError::AlreadyInitialized`]
    /// - [`SdlError::SysError`]
    #[doc(alias = "SDL_InitSubSystem(SDL_INIT_AUDIO)")]
    #[inline]
    pub fn audio(&self) -> Result<SdlSubsystem<Audio>, SdlError> {
        self.init_subsystem(SdlSubsystemFlag::Audio)
    }

    /// Initializes the audio subsystem with the provided driver.
    /// 
    /// Note: You should call [`audio`] unless you have a need
    /// to designate a specific audio driver to be used.
    /// 
    /// ### Errors
    /// - [`SdlError::AlreadyInitialized`]
    /// - [`SdlError::SysError`]
    #[doc(alias = "SDL_AudioInit")]
    pub fn audio_with_driver(&self, driver_name: &str) -> Result<SdlSubsystem<Audio>, SdlError> {
        if AUDIO_INITIALIZED.load(Ordering::SeqCst) {
            return Err(SdlError::AlreadyInitialized(format!("The Audio subsystem is already initialized.")));
        }

        let driver_name = CString::new(driver_name)?;

        #[cfg(feature = "log")] debug!("Calling 'SDL_AudioInit'");
        if unsafe { SDL_AudioInit(driver_name.as_ptr()) == 0 } {
            AUDIO_INITIALIZED.store(true, Ordering::SeqCst);

            Ok(SdlSubsystem {
                t: Default::default(),
                subsystem: SdlSubsystemFlag::Audio,
                quitter: Some(SDL_AudioQuit)
            })
        } else {
            Err(SdlError::SysError(get_sys_error().unwrap()))
        }
    }
}
