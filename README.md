# hd44780-controller

Controller for [HD44780] LCD Controller written in [Rust] which aims to suppports 
and both sync and async with `no_std`.

Currently, only async `Controller` and [embedded-hal-async] based `pcf8574` I2c I/O Expander `AsyncDevice` are implemented.

You can add support for more I/O methods by implementing custom `Device` trait for sync and `AsyncDevice` trait for async.

Support for sync `Controller`, [embedded-hal] based `pcf8574` sync `Device` and CGRAM, CGROM will be added later.

## Licenses

All code available under the [MIT] license.
You can find a copy of the license in the [LICENSE] file.

[HD44780]: https://en.wikipedia.org/wiki/Hitachi_HD44780_LCD_controller/
[LICENSE]: LICENSE
[embedded-hal]: https://github.com/rust-embedded/embedded-hal
[embedded-hal-async]: https://github.com/rust-embedded/embedded-hal/tree/master/embedded-hal-async
[MIT]: https://opensource.org/licenses/MIT
