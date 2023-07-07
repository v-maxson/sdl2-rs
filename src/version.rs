use crate::sys::*;
use semver::Version;

/// The SDL2 version used for this library.
pub const SDL2_VERSION: Version = Version::new(SDL_MAJOR_VERSION as _, SDL_MINOR_VERSION as _, SDL_PATCHLEVEL as _);
