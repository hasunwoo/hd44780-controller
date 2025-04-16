use crate::device::*;

use super::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DisplayOnOff {
    pub display: bool,
    pub cursor: bool,
    pub blinking: bool,
}

impl DisplayOnOff {
    fn code(&self) -> u8 {
        let mut code = 0x08;

        if self.display {
            code |= 0x1 << 2;
        }
        if self.cursor {
            code |= 0x1 << 1;
        }
        if self.blinking {
            code |= 0x1 << 0;
        }

        code
    }
}

impl Default for DisplayOnOff {
    fn default() -> Self {
        Self {
            display: true,
            cursor: false,
            blinking: false,
        }
    }
}

impl SyncCommand for DisplayOnOff {
    type Ret = ();

    type Err = super::Error;

    fn execute<D: SyncDevice + ?Sized>(&self, dev: &mut D) -> Result<Self::Ret, Self::Err> {
        dev.write_byte(RegisterSelectMode::Command, self.code())
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us(50);
        Ok(())
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl AsyncCommand for DisplayOnOff {
    type Ret = ();

    type Err = super::Error;

    async fn execute_async<D: AsyncDevice + ?Sized>(
        &self,
        dev: &mut D,
    ) -> Result<Self::Ret, Self::Err> {
        dev.write_byte_async(RegisterSelectMode::Command, self.code())
            .await
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us_async(50).await;
        Ok(())
    }
}
