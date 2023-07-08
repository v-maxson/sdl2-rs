use super::{SdlSubsystem, markers::Timer};
use crate::context::SdlContext;
use crate::{sys::*, error::SdlError, utils::get_sys_error};

impl SdlSubsystem<Timer> {

    /// Get the number of milliseconds since the [`SdlContext`] was initialized.
    /// 
    /// This value wraps if the program runs for more that ~49 days.
    #[doc(alias = "SDL_GetTicks")]
    pub fn get_ticks(&self) -> u32 {
        unsafe { SDL_GetTicks() }
    }

    /// Get the current value of the high resolution counter.
    #[doc(alias = "SDL_GetPerformanceCounter")]
    pub fn get_performance_counter(&self) -> u64 {
        unsafe { SDL_GetPerformanceCounter() }
    }

    /// Get the count per second of the high resolution counter.
    #[doc(alias = "SDL_GetPerformanceFrequency")]
    pub fn get_performance_freq(&self) -> u64 {
        unsafe { SDL_GetPerformanceFrequency() }
    }
 
    /// This functions blocks execution for the given amount of milliseconds.
    /// 
    /// This function waits a specified number of milliseconds before returning. It
    /// waits at least the specified time, but possibly longer due to OS
    /// scheduling.
    #[doc(alias = "SDL_Delay")]
    pub fn delay(&self, ms: u32) {
        unsafe { SDL_Delay(ms) }
    }

    /// This function is not currently implemented.
    /// See [`SDL_AddTimer`].
    #[doc(alias = "SDL_AddTimer")]
    pub fn add_timer(&self) { todo!() }

    /// This function is not currently implemented.
    /// See [`SDL_RemoveTimer`].
    #[doc(alias = "SDL_RemoveTimer")]
    pub fn remove_timer(&self) { todo!() }
}