use std::{marker::PhantomData, sync::atomic::{AtomicBool, Ordering}};
use crate::sys::*;

pub mod markers;

pub mod timer;

pub(crate) static TIMER_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static AUDIO_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static VIDEO_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static JOYSTICK_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static HAPTIC_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static GAME_CONTROLLER_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static EVENTS_INITIALIZED: AtomicBool = AtomicBool::new(false);
pub(crate) static SENSOR_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SdlSubsystemFlag {
    Timer = SDL_INIT_TIMER,
    Audio = SDL_INIT_AUDIO,
    Video = SDL_INIT_VIDEO,
    Joystick = SDL_INIT_JOYSTICK,
    Haptic = SDL_INIT_HAPTIC,
    GameController = SDL_INIT_GAMECONTROLLER,
    Events = SDL_INIT_EVENTS,
    Sensor = SDL_INIT_SENSOR
}

impl SdlSubsystemFlag {
    /// Returns true if the subsystem associated with this flag is initialized.
    pub fn initialized(&self) -> bool {
        let initialized = match self {
            SdlSubsystemFlag::Timer => &TIMER_INITIALIZED,
            SdlSubsystemFlag::Audio => &AUDIO_INITIALIZED,
            SdlSubsystemFlag::Video => &VIDEO_INITIALIZED,
            SdlSubsystemFlag::Joystick => &JOYSTICK_INITIALIZED,
            SdlSubsystemFlag::Haptic => &HAPTIC_INITIALIZED,
            SdlSubsystemFlag::GameController => &GAME_CONTROLLER_INITIALIZED,
            SdlSubsystemFlag::Events => &EVENTS_INITIALIZED,
            SdlSubsystemFlag::Sensor => &SENSOR_INITIALIZED,
        };

        initialized.load(Ordering::SeqCst)
    }

    /// Returns a reference to the [`AtomicBool`] associated with this flag.
    pub(crate) fn initialized_raw(&self) -> &AtomicBool {
        match self {
            SdlSubsystemFlag::Timer => &TIMER_INITIALIZED,
            SdlSubsystemFlag::Audio => &AUDIO_INITIALIZED,
            SdlSubsystemFlag::Video => &VIDEO_INITIALIZED,
            SdlSubsystemFlag::Joystick => &JOYSTICK_INITIALIZED,
            SdlSubsystemFlag::Haptic => &HAPTIC_INITIALIZED,
            SdlSubsystemFlag::GameController => &GAME_CONTROLLER_INITIALIZED,
            SdlSubsystemFlag::Events => &EVENTS_INITIALIZED,
            SdlSubsystemFlag::Sensor => &SENSOR_INITIALIZED,
        }
    }
}

pub struct SdlSubsystem<T: markers::SdlSubsystemMarker>(pub(crate) PhantomData<T>, pub(crate) SdlSubsystemFlag);

impl<T: markers::SdlSubsystemMarker> Drop for SdlSubsystem<T> {
    fn drop(&mut self) {
        let initialized = self.1.initialized_raw();
        initialized.store(false, Ordering::SeqCst);

        #[cfg(feature = "log")] debug!("Calling 'SDL_QuitSubSystem({:?})'", self.1);
        unsafe {
            SDL_QuitSubSystem(self.1 as _)
        }
    }
}
