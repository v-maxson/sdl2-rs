use std::time::Duration;
use super::{SdlSubsystem, markers::Timer};
use crate::sys::*;

impl SdlSubsystem<Timer> {

    /// Get a [`Duration`] representing the amount of time since the [`SdlContext`] was initialized.
    /// 
    /// This value wraps if the program runs for more that ~49 days.
    #[doc(alias = "SDL_GetTicks")]
    pub fn get_ticks(&self) -> Duration {
        #[cfg(feature = "log")] debug!("Calling 'SDL_GetTicks()'");
        Duration::from_millis(unsafe { SDL_GetTicks() } as _)
    }

    /// Get the current value of the high resolution counter.
    #[doc(alias = "SDL_GetPerformanceCounter")]
    pub fn get_performance_counter(&self) -> u64 {
        #[cfg(feature = "log")] debug!("Calling 'SDL_GetPerformanceCounter()'");
        unsafe { SDL_GetPerformanceCounter() }
    }

    /// Get the count per second of the high resolution counter.
    #[doc(alias = "SDL_GetPerformanceFrequency")]
    pub fn get_performance_freq(&self) -> u64 {
        #[cfg(feature = "log")] debug!("Calling 'SDL_GetPerformanceFrequency()'");
        unsafe { SDL_GetPerformanceFrequency() }
    }
 
    /// This functions blocks execution for the given amount of milliseconds.
    /// 
    /// This function waits a specified number of milliseconds before returning. It
    /// waits at least the specified time, but possibly longer due to OS
    /// scheduling.
    #[doc(alias = "SDL_Delay")]
    pub fn delay(&self, ms: u32) {
        #[cfg(feature = "log")] debug!("Calling 'SDL_Delay()'");
        unsafe { SDL_Delay(ms) }
    }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_AddTimer`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_timer.h).
    #[doc(alias = "SDL_AddTimer")]
    pub fn add_timer(&self) { todo!() }

    /// This function is not currently unimplemented and will __panic__ if called.
    /// See [`SDL_RemoveTimer`](https://github.com/libsdl-org/SDL/blob/SDL2/include/SDL_timer.h).
    #[doc(alias = "SDL_RemoveTimer")]
    pub fn remove_timer(&self) { todo!() }
}