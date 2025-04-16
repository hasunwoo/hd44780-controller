use crate::device::*;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteChar(pub u8);

impl SyncCommand for WriteChar {
    type Ret = ();

    type Err = super::Error;

    fn execute<D: SyncDevice + ?Sized>(&self, dev: &mut D) -> Result<Self::Ret, Self::Err> {
        dev.write_byte(RegisterSelectMode::Data, self.0)
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us(50);

        Ok(())
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl AsyncCommand for WriteChar {
    type Ret = ();

    type Err = super::Error;

    async fn execute_async<D: AsyncDevice + ?Sized>(
        &self,
        dev: &mut D,
    ) -> Result<Self::Ret, Self::Err> {
        dev.write_byte_async(RegisterSelectMode::Data, self.0)
            .await
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us_async(50).await;

        Ok(())
    }
}
