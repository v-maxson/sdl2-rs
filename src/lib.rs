#![allow(unused)]

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

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
    use ctor::ctor;
    use simplelog::{SimpleLogger, Config};
    use crate::{SdlContext, SdlError};

    // This ensures there is a logger initialized for use with the `log` crate in all tests. 
    // You should NOT use the `ctor` crate with anything that calls into Rust's standard library.
    #[ctor]
    static LOGGER: Box<SimpleLogger> = SimpleLogger::new(log::LevelFilter::Off, Config::default());

    #[test]
    fn timer_test() -> Result<(), SdlError> {
        let ctx = SdlContext::new()?;
        let timer = ctx.timer()?;

        timer.delay(3000);

        println!("Time Since SdlContext initialization: {}ms", timer.get_ticks().as_millis());

        Ok(())
    }
}
