pub trait Device {
    fn set_register_select(&mut self, mode: RegisterSelectMode);
    fn set_rw(&mut self, mode: RWMode);
    fn set_enable(&mut self, enabled: bool);
    fn set_backlight(&mut self, enabled: bool);
    fn set_data_nibble(&mut self, data: u8);
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub trait AsyncDevice: Device {
    type Err;

    async fn delay_us_async(&mut self, us: u32);
    async fn flush_async(&mut self) -> Result<(), Self::Err>;

    async fn write_nibble_async(
        &mut self,
        mode: RegisterSelectMode,
        data: u8,
    ) -> Result<(), Self::Err> {
        self.set_register_select(mode);
        self.set_rw(RWMode::Write);
        self.set_data_nibble(data);

        self.set_enable(true);
        self.flush_async().await?;
        self.set_enable(false);
        self.flush_async().await?;

        Ok(())
    }

    async fn write_byte_async(
        &mut self,
        mode: RegisterSelectMode,
        byte: u8,
    ) -> Result<(), Self::Err> {
        let high = (byte >> 4) & 0x0F;
        let low = byte & 0x0F;

        self.write_nibble_async(mode, high).await?;
        self.write_nibble_async(mode, low).await?;
        Ok(())
    }
}

pub trait SyncDevice: Device {
    type Err;

    fn delay_us(&mut self, us: u32);
    fn flush(&mut self) -> Result<(), Self::Err>;

    fn write_nibble(&mut self, mode: RegisterSelectMode, data: u8) -> Result<(), Self::Err> {
        self.set_register_select(mode);
        self.set_rw(RWMode::Write);
        self.set_data_nibble(data);

        self.set_enable(true);
        self.flush()?;
        self.set_enable(false);
        self.flush()?;

        Ok(())
    }

    fn write_byte(&mut self, mode: RegisterSelectMode, byte: u8) -> Result<(), Self::Err> {
        let high = (byte >> 4) & 0x0F;
        let low = byte & 0x0F;

        self.write_nibble(mode, high)?;
        self.write_nibble(mode, low)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RegisterSelectMode {
    Command,
    Data,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RWMode {
    Read,
    Write,
}
