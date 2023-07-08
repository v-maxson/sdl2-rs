use std::{marker::PhantomData, sync::atomic::{AtomicBool, Ordering}};
use crate::{sys::*, error::SdlError, utils::get_sys_error};

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
pub enum Subsystem {
    Timer = SDL_INIT_TIMER,
    Audio = SDL_INIT_AUDIO,
    Video = SDL_INIT_VIDEO,
    Joystick = SDL_INIT_JOYSTICK,
    Haptic = SDL_INIT_HAPTIC,
    GameController = SDL_INIT_GAMECONTROLLER,
    Events = SDL_INIT_EVENTS,
    Sensor = SDL_INIT_SENSOR
}

pub struct SdlSubsystem<T: markers::SdlSubsystemMarker>(pub(crate) PhantomData<T>, pub(crate) Subsystem);

impl<T: markers::SdlSubsystemMarker> Drop for SdlSubsystem<T> {
    fn drop(&mut self) {
        let initialized = match self.1 {
            Subsystem::Timer => &TIMER_INITIALIZED,
            Subsystem::Audio => &AUDIO_INITIALIZED,
            Subsystem::Video => &VIDEO_INITIALIZED,
            Subsystem::Joystick => &JOYSTICK_INITIALIZED,
            Subsystem::Haptic => &HAPTIC_INITIALIZED,
            Subsystem::GameController => &GAME_CONTROLLER_INITIALIZED,
            Subsystem::Events => &EVENTS_INITIALIZED,
            Subsystem::Sensor => &SENSOR_INITIALIZED,
        };
        initialized.store(false, Ordering::SeqCst);

        unsafe {
            SDL_QuitSubSystem(self.1 as _)
        }
    }
}
