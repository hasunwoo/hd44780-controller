# hd44780-controller

A `no_std`-compatible controller for the [HD44780] LCD controller written in [Rust].  
This library supports **both sync and async** interfaces and is designed to be minimal and flexible.

This crate does **not depend on any specific I/O implementation**. Instead, you must provide your own implementation of the `Device` (sync) or `AsyncDevice` (async) trait.

Currently, only the async `Controller` and the `AsyncDevice` implementation for [embedded-hal-async]-based `PCF8574` I²C I/O expander are available.

Support for a sync `Controller` and CGRAM/CGROM manipulation will be added in the future.

---

## Feature Flags

By default, **no features are enabled**. To do anything useful, you'll need to enable at least one `Device` implementation.

For example, to use an async PCF8574 device over I²C, enable the `pcf8574-device-async` feature.

- `default`: (empty)  
- `async`: Enables async support.
- `fmt`: Enables formatted text output via `heapless::String`.
- `i2c-expander-device`: Enables support for generic I²C expander devices (`embedded-hal` required).
- `i2c-expander-device-async`: Enables async I²C expander support (`async` + `embedded-hal-async` required).
- `pcf8574-device`: Enables support for the PCF8574 I²C I/O expander.
- `pcf8574-device-async`: Enables async support for PCF8574.
- `all`: Enables all of the above.

---

## License

This project is licensed under the [MIT] license.  
See the [LICENSE] file for more information.

---

[HD44780]: https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller/
[Rust]: https://www.rust-lang.org/
[MIT]: https://opensource.org/licenses/MIT
[LICENSE]: LICENSE
[embedded-hal]: https://github.com/rust-embedded/embedded-hal
[embedded-hal-async]: https://github.com/rust-embedded/embedded-hal/tree/master/embedded-hal-async
