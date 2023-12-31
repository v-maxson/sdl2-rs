[SDL2]: https://github.com/libsdl-org/SDL/tree/SDL2
[sdl2]: https://github.com/Rust-SDL2/rust-sdl2
[sdl2-sys]: https://github.com/Rust-SDL2/rust-sdl2/tree/master/sdl2-sys
[log]: https://github.com/rust-lang/log

# sdl2-rs
A safe interface to [SDL2][SDL2].

> **Disclaimer**: This library is mainly created as part of my portfolio. For an alternative
> that is well maintained and tested, see the [`sdl2`][sdl2] crate. 
> In fact, this library is built on top of the [`sdl2-sys`][sdl2-sys] bindings that
> [`sdl2`][sdl2] generates.

> **Note**: Until a `1.0` release is reached, this library will likely undergo major API changes
> without warning. Also, until basic API structure is fully laid out and in a state that I
> am happy with, this library will remain at version `0.0.0`.

### Features 
- All features from the [`sdl2-sys`][sdl2-sys] crate are inherited.
  - `bundled`
  - `static-link`
  - `use-vcpkg`
  - `use-bindgen`
  - `use-pkgconfig`
  - `use_mac_framework`
  - `ttf`
  - `flate2`
  - `image`
  - `mixer`
  - `gfx`
- `log`
  - Enables debug logging via the [`log`][log] crate.
  - Note: The [`log`][log] crate requires an implementation
    to write logs to console, files, etc.
  - Another Note: This crate logs with EVERY call to foreign code.