#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    i2c::{self, I2c},
    peripherals::I2C0,
};
use embassy_time::{Delay, Timer};
use hd44780_controller::{
    controller::{
        Controller,
        config::{InitialConfig, RuntimeConfig},
    },
    device::pcf8574::PCF8574Device,
    lcd_println,
};
use panic_halt as _;

bind_interrupts!(struct Irqs {
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let i2c0_bus = i2c::I2c::new_async(p.I2C0, p.PIN_21, p.PIN_20, Irqs, i2c::Config::default());

    let device = PCF8574Device::new(i2c0_bus, 0x27, Delay);

    let mut controller =
        Controller::<PCF8574Device<I2c<'static, I2C0, i2c::Async>, Delay>>::new_async(
            device,
            InitialConfig::default(),
            RuntimeConfig::default(),
        )
        .init()
        .await
        .unwrap();

    lcd_println!(controller, line = 0, "Hello World!")
        .await
        .unwrap();

    let mut count = 0u8;
    loop {
        lcd_println!(controller, line = 1, "count: {count}")
            .await
            .unwrap();
        count = count.wrapping_add(1);
        Timer::after_millis(500).await;
    }
}
