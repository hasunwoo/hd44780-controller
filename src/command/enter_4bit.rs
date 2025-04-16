use crate::device::*;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enter4Bit;

impl SyncCommand for Enter4Bit {
    type Ret = ();

    type Err = super::Error;

    fn execute<D: SyncDevice + ?Sized>(&self, dev: &mut D) -> Result<Self::Ret, Self::Err> {
        dev.delay_us(50000);

        dev.write_nibble(RegisterSelectMode::Command, 0x3)
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us(4500);

        dev.write_nibble(RegisterSelectMode::Command, 0x3)
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us(150);

        dev.write_nibble(RegisterSelectMode::Command, 0x2)
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us(150);

        Ok(())
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl AsyncCommand for Enter4Bit {
    type Ret = ();

    type Err = super::Error;

    async fn execute_async<D: AsyncDevice + ?Sized>(
        &self,
        dev: &mut D,
    ) -> Result<Self::Ret, Self::Err> {
        dev.delay_us_async(50000).await;

        dev.write_nibble_async(RegisterSelectMode::Command, 0x3)
            .await
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us_async(4500).await;

        dev.write_nibble_async(RegisterSelectMode::Command, 0x3)
            .await
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us_async(150).await;

        dev.write_nibble_async(RegisterSelectMode::Command, 0x2)
            .await
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us_async(150).await;

        Ok(())
    }
}
