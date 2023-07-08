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

#[cfg(test)]
mod tests {
    use crate::{SdlContext, SdlError};


    #[test]
    fn timer_test() -> Result<(), SdlError> {
        let ctx = SdlContext::new()?;
        let timer = ctx.timer()?;

        timer.delay(3000);

        println!("Time Since SdlContext initialization: {}ms", timer.get_ticks());

        Ok(())
    }
}
