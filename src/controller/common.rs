use core::marker::PhantomData;

use crate::{command, device::*};

use super::config::{InitialConfig, RuntimeConfig};

pub mod state {
    pub struct Empty;
    pub struct Uninit;
    pub struct Init;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    CommandError,
    OutOfBounds,
}

impl From<command::Error> for Error {
    fn from(_value: command::Error) -> Self {
        Error::CommandError
    }
}

#[allow(unused)]
pub struct Controller<D, S = state::Empty> {
    pub(super) device: D,
    pub(super) initial_config: InitialConfig,
    pub(super) runtime_config: RuntimeConfig,
    pub(super) _state: PhantomData<S>,
}

impl<D> Controller<D, state::Empty> {
    pub fn new<Dev: Device>(
        device: Dev,
        initial_config: InitialConfig,
        runtime_config: RuntimeConfig,
    ) -> Controller<Dev, state::Uninit> {
        Controller {
            device,
            initial_config,
            runtime_config,
            _state: PhantomData,
        }
    }

    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn new_async<Dev: AsyncDevice>(
        device: Dev,
        initial_config: InitialConfig,
        runtime_config: RuntimeConfig,
    ) -> Controller<Dev, state::Uninit> {
        Controller {
            device,
            initial_config,
            runtime_config,
            _state: PhantomData,
        }
    }
}

#[cfg(feature = "fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
#[macro_export]
macro_rules! lcd_print {
    ($lcd:expr, $($arg:tt)*) => {
        $lcd.write_fmt::<128>(format_args!($($arg)*))
    };
}
