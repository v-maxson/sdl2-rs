#![allow(unused)]

// Reexport sdl2-sys.
pub use sdl2_sys as sys;

// Private Modules
pub(crate) mod utils;

// Public Modules
mod context; pub use context::*;
mod error; pub use error::*;
pub mod subsystems;
pub mod version;
