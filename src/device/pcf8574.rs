use super::i2c_expander_device::{I2cExpanderDevice, I2cPacketAssembler, State};

pub type PCF8574Device<I, D> = I2cExpanderDevice<I, D, PCF8574>;

pub struct PCF8574;

impl I2cPacketAssembler for PCF8574 {
    type Payload = [u8; 1];

    fn assemble_i2c_packet(state: &State) -> Self::Payload {
        let mut packet = 0u8;

        // registers
        if state.rs {
            packet |= 1 << 0;
        }
        if state.rw {
            packet |= 1 << 1;
        }
        if state.e {
            packet |= 1 << 2;
        }
        if state.bl {
            packet |= 1 << 3;
        }

        // data
        packet |= (state.data & 0x0f) << 4;

        [packet]
    }
}
