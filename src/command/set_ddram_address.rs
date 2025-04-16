use crate::device::*;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetDDRamAddress(pub u8);

impl SetDDRamAddress {
    fn code(&self) -> u8 {
        0x80 | (self.0 & 0x7f)
    }
}

impl SyncCommand for SetDDRamAddress {
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
impl AsyncCommand for SetDDRamAddress {
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
