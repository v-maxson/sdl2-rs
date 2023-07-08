use crate::sys::*;

/// Marker for SDL subsystems.
pub trait SdlSubsystemMarker {}

pub struct Timer; impl SdlSubsystemMarker for Timer {}
pub struct Audio; impl SdlSubsystemMarker for Audio {}
pub struct Video; impl SdlSubsystemMarker for Video {}
pub struct Joystick; impl SdlSubsystemMarker for Joystick {}
pub struct Haptic; impl SdlSubsystemMarker for Haptic {}
pub struct GameController; impl SdlSubsystemMarker for GameController {}
pub struct Events; impl SdlSubsystemMarker for Events {}
pub struct Sensor; impl SdlSubsystemMarker for Sensor {}