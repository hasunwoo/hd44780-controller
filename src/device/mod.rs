mod common;
pub use common::*;

mod command_ext;
pub use command_ext::*;

#[cfg(any(feature = "i2c-expander-device", feature = "i2c-expander-device-async"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "i2c-expander-device", feature = "i2c-expander-device-async")))
)]
pub mod i2c_expander_device;

#[cfg(any(feature = "pcf8574-device", feature = "pcf8574-device-async"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "pcf8574-device", feature = "pcf8574-device-async")))
)]
pub mod pcf8574;
