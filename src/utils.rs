use std::ffi::CStr;
use crate::sys::*;

pub fn get_sys_error() -> Option<String> {
    #[cfg(feature = "log")]
    debug!("Calling 'SDL_GetError()'");
    unsafe {
        CStr::from_ptr(SDL_GetError()).to_str().ok().map(|s| s.to_string())
    }
}
