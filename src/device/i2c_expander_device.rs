use core::marker::PhantomData;

use super::*;

pub trait I2cPacketAssembler {
    type Payload: AsRef<[u8]>;

    fn assemble_i2c_packet(state: &State) -> Self::Payload;
}

pub struct I2cExpanderDevice<I, D, P> {
    i2c: I,
    i2c_address: u8,
    delay: D,
    state: State,
    _packet_assembler: PhantomData<P>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct State {
    // register select
    pub rs: bool,
    // read/write
    pub rw: bool,
    // enable
    pub e: bool,
    // backlight
    pub bl: bool,
    // 4 bit data
    pub data: u8,
}

impl<I, D, P> I2cExpanderDevice<I, D, P> {
    pub fn new(i2c: I, i2c_address: u8, delay: D) -> Self {
        let state = State {
            rs: false,
            rw: false,
            e: false,
            bl: true,
            data: 0,
        };
        Self {
            i2c,
            i2c_address,
            delay,
            state,
            _packet_assembler: PhantomData,
        }
    }

    pub fn release(self) -> (I, D) {
        (self.i2c, self.delay)
    }

    pub fn i2c_address(&self) -> u8 {
        self.i2c_address
    }
}

impl<I, D, P> Device for I2cExpanderDevice<I, D, P> {
    fn set_register_select(&mut self, mode: RegisterSelectMode) {
        self.state.rs = match mode {
            RegisterSelectMode::Command => false,
            RegisterSelectMode::Data => true,
        };
    }

    fn set_rw(&mut self, mode: RWMode) {
        self.state.rw = match mode {
            RWMode::Read => true,
            RWMode::Write => false,
        };
    }

    fn set_enable(&mut self, enabled: bool) {
        self.state.e = enabled;
    }

    fn set_backlight(&mut self, enabled: bool) {
        self.state.bl = enabled;
    }

    fn set_data_nibble(&mut self, data: u8) {
        self.state.data = data;
    }
}

#[cfg(all(feature = "async", feature = "i2c-expander-device-async"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "async", feature = "i2c-expander-device-async")))
)]
impl<I, D, P> AsyncDevice for I2cExpanderDevice<I, D, P>
where
    I: embedded_hal_async::i2c::I2c,
    D: embedded_hal_async::delay::DelayNs,
    P: I2cPacketAssembler,
{
    type Err = I::Error;

    async fn delay_us_async(&mut self, us: u32) {
        self.delay.delay_us(us).await;
    }

    async fn flush_async(&mut self) -> Result<(), Self::Err> {
        let packet = P::assemble_i2c_packet(&self.state);
        self.i2c.write(self.i2c_address, packet.as_ref()).await
    }
}

#[cfg(feature = "i2c-expander-device")]
#[cfg_attr(docsrs, doc(cfg(feature = "i2c-expander-device")))]
impl<I, D, P> SyncDevice for I2cExpanderDevice<I, D, P>
where
    I: embedded_hal::i2c::I2c,
    D: embedded_hal::delay::DelayNs,
    P: I2cPacketAssembler,
{
    type Err = I::Error;

    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us);
    }

    fn flush(&mut self) -> Result<(), Self::Err> {
        let packet = P::assemble_i2c_packet(&self.state);
        self.i2c.write(self.i2c_address, packet.as_ref())
    }
}
