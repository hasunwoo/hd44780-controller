use crate::device::*;

use super::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ReturnHome;

impl SyncCommand for ReturnHome {
    type Ret = ();

    type Err = super::Error;

    fn execute<D: SyncDevice + ?Sized>(&self, dev: &mut D) -> Result<Self::Ret, Self::Err> {
        dev.write_byte(RegisterSelectMode::Command, 0x02)
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us(2000);
        Ok(())
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl AsyncCommand for ReturnHome {
    type Ret = ();

    type Err = super::Error;

    async fn execute_async<D: AsyncDevice + ?Sized>(
        &self,
        dev: &mut D,
    ) -> Result<Self::Ret, Self::Err> {
        dev.write_byte_async(RegisterSelectMode::Command, 0x02)
            .await
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us_async(2000).await;
        Ok(())
    }
}
