#![no_std]
#![no_main]

// use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{self, Config as I2cConfig, InterruptHandler as I2cInterruptHandler};
use embassy_rp::peripherals::{I2C1, USB};
use embassy_rp::spi::{self, Config as SpiConfig};
use embassy_rp::usb::{Driver, InterruptHandler as UsbInterruptHandler};
use embassy_time::{Duration, Timer};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::{Builder, Config as UsbConfig};
use {defmt_rtt as _, panic_halt as _};

mod camera;
mod ov2640_regs;

use camera::Camera;

// #[link_section = ".boot2"]
// #[used]
// pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
    I2C1_IRQ => I2cInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // USB Setup
    let driver = Driver::new(p.USB, Irqs);
    let mut config = UsbConfig::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Antigravity");
    config.product = Some("Arducam FW");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    static CONFIG_DESC: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
    static BOS_DESC: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
    static DEVICE_DESC: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
    static CONTROL_BUF: static_cell::StaticCell<[u8; 64]> = static_cell::StaticCell::new();
    static STATE: static_cell::StaticCell<State> = static_cell::StaticCell::new();

    let mut builder = Builder::new(
        driver,
        config,
        DEVICE_DESC.init([0; 256]),
        CONFIG_DESC.init([0; 256]),
        BOS_DESC.init([0; 256]),
        CONTROL_BUF.init([0; 64]),
    );

    let mut class = CdcAcmClass::new(&mut builder, STATE.init(State::new()), 64);

    let usb = builder.build();
    spawner.spawn(usb_task(usb)).unwrap();

    // LED for heartbeat
    let mut led = Output::new(p.PIN_13, Level::Low);

    // I2C Setup (SDA=GPIO2, SCL=GPIO3, I2C1)
    let sda = p.PIN_2;
    let scl = p.PIN_3;
    let i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, I2cConfig::default());

    // SPI Setup (SCK=18, MOSI=19, MISO=20, CS=1, SPI0)
    let sck = p.PIN_18;
    let mosi = p.PIN_19;
    let miso = p.PIN_20;

    // Config
    let mut spi_config = SpiConfig::default();
    spi_config.frequency = 8_000_000;

    let spi = spi::Spi::new(p.SPI0, sck, mosi, miso, p.DMA_CH0, p.DMA_CH1, spi_config);
    let cs = Output::new(p.PIN_1, Level::High);

    // Camera Init
    let mut cam = Camera::new(spi, i2c, cs);

    // Allow USB to connect
    Timer::after(Duration::from_secs(1)).await;

    match cam.init().await {
        Ok(_) => {
            // Blink faster on success
            for _ in 0..5 {
                led.toggle();
                Timer::after(Duration::from_millis(100)).await;
            }
        }
        Err(_) => {
            // Blink SOS on fail
            loop {
                led.set_high();
                Timer::after(Duration::from_millis(500)).await;
                led.set_low();
                Timer::after(Duration::from_millis(500)).await;
            }
        }
    }

    loop {
        led.toggle();

        cam.start_capture().await;

        // Wait for capture
        while !cam.capture_done().await {
            Timer::after(Duration::from_millis(10)).await;
        }

        let len = cam.read_fifo_length().await;
        if len > 0 && len < 0x5FFFF {
            // Send Sync Marker
            let sync = [0xAA, 0x55, 0xAA, 0x55];
            let _ = class.write_packet(&sync).await;

            // Read FIFO and send over USB
            cam.burst_read_start().await;

            let mut buf = [0u8; 64];
            let mut remaining = len;
            while remaining > 0 {
                let chunk = if remaining > 64 {
                    64
                } else {
                    remaining as usize
                };
                cam.burst_read(&mut buf[0..chunk]).await;

                // Send to USB (blocking wait if full)
                let _ = class.write_packet(&buf[0..chunk]).await;

                remaining -= chunk as u32;
            }

            cam.burst_read_end();
        }

        cam.clear_fifo_flag().await;

        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn usb_task(mut usb: embassy_usb::UsbDevice<'static, Driver<'static, USB>>) {
    usb.run().await;
}
