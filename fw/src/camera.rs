use embassy_rp::gpio::Output;
use embassy_time::{Duration, Timer};
use embedded_hal_async::i2c::I2c as I2cTrait;
use embedded_hal_async::spi::SpiBus;

use crate::ov2640_regs;

const ARDUCHIP_TEST1: u8 = 0x00;
const ARDUCHIP_FIFO: u8 = 0x04;
const FIFO_CLEAR_MASK: u8 = 0x01;
const FIFO_START_MASK: u8 = 0x02;
const ARDUCHIP_TRIG: u8 = 0x41;
const CAP_DONE_MASK: u8 = 0x08;
const BURST_FIFO_READ: u8 = 0x3C;

const OV2640_CHIPID_HIGH: u8 = 0x0A;
const OV2640_CHIPID_LOW: u8 = 0x0B;

const SENSOR_ADDR: u8 = 0x30;

pub struct Camera<'d, SPI: SpiBus, I2C: I2cTrait> {
    spi: SPI,
    i2c: I2C,
    cs: Output<'d>,
}

impl<'d, SPI: SpiBus, I2C: I2cTrait> Camera<'d, SPI, I2C> {
    pub fn new(spi: SPI, i2c: I2C, cs: Output<'d>) -> Self {
        Self { spi, i2c, cs }
    }

    pub async fn init(&mut self) -> Result<(), ()> {
        // Reset CPLD
        self.write_reg(0x07, 0x80).await;
        Timer::after(Duration::from_millis(100)).await;
        self.write_reg(0x07, 0x00).await;
        Timer::after(Duration::from_millis(100)).await;

        // Check SPI
        loop {
            self.write_reg(ARDUCHIP_TEST1, 0x55).await;
            let val = self.read_reg(ARDUCHIP_TEST1).await;
            if val == 0x55 {
                break;
            }
            Timer::after(Duration::from_millis(1000)).await;
        }

        // Check Sensor (OV2640)
        let _vid = self.rd_sensor_reg8_8(OV2640_CHIPID_HIGH).await;
        let _pid = self.rd_sensor_reg8_8(OV2640_CHIPID_LOW).await;
        // Could verify VID/PID here (0x26, 0x42)

        // Init Sensor
        self.wr_sensor_reg8_8(0xff, 0x01).await;
        self.wr_sensor_reg8_8(0x12, 0x80).await; // Reset
        Timer::after(Duration::from_millis(100)).await;

        self.wr_sensor_regs(ov2640_regs::OV2640_QVGA).await;
        // self.wr_sensor_regs(ov2640_regs::OV2640_64X64_NATIVE).await;

        self.wr_sensor_reg8_8(0xff, 0x01).await;
        self.wr_sensor_reg8_8(0x15, 0x00).await;

        Timer::after(Duration::from_millis(1000)).await;
        self.clear_fifo_flag().await;

        Ok(())
    }

    pub async fn start_capture(&mut self) {
        self.write_reg(ARDUCHIP_FIFO, FIFO_START_MASK).await;
    }

    pub async fn clear_fifo_flag(&mut self) {
        self.write_reg(ARDUCHIP_FIFO, FIFO_CLEAR_MASK).await;
    }

    pub async fn capture_done(&mut self) -> bool {
        let val = self.read_reg(ARDUCHIP_TRIG).await;
        (val & CAP_DONE_MASK) != 0
    }

    pub async fn read_fifo_length(&mut self) -> u32 {
        let len1 = self.read_reg(0x42).await as u32;
        let len2 = self.read_reg(0x43).await as u32;
        let len3 = self.read_reg(0x44).await as u32;
        ((len3 & 0x7f) << 16) | (len2 << 8) | len1
    }

    pub async fn burst_read_start(&mut self) {
        self.cs.set_low();
        let mut rx = [0u8; 1];
        let tx = [BURST_FIFO_READ];
        let _ = self.spi.transfer(&mut rx, &tx).await;
        // Read Dummy Byte to skip it
        let mut dummy = [0u8; 1];
        let zero = [0u8; 1];
        let _ = self.spi.transfer(&mut dummy, &zero).await;
    }

    pub async fn burst_read(&mut self, buf: &mut [u8]) {
        let tx = [0u8; 64];
        // We must handle chunks if buf is larger than our stack buffer,
        // but main.rs passes max 64 bytes.
        if buf.len() <= tx.len() {
            let _ = self.spi.transfer(buf, &tx[0..buf.len()]).await;
        } else {
            // Fallback or panic? For now assume caller respects chunk size or implementation pads
            // Actually, create a dynamic zero filler or chunk it here?
            // Safest for embassy is strict length match.
            // We'll iterate.
            for chunk in buf.chunks_mut(64) {
                let _ = self.spi.transfer(chunk, &tx[0..chunk.len()]).await;
            }
        }
    }

    pub fn burst_read_end(&mut self) {
        self.cs.set_high();
    }

    // SPI Register Access
    async fn write_reg(&mut self, addr: u8, val: u8) {
        self.cs.set_low();
        // write requires just a write buffer? No, embassy spi write reads and discards.
        // But transfer is explicit.
        let _ = self.spi.write(&[addr | 0x80, val]).await;
        self.cs.set_high();
    }

    async fn read_reg(&mut self, addr: u8) -> u8 {
        self.cs.set_low();
        let mut rx = [0u8; 2];
        let tx = [addr & 0x7F, 0x00];
        let _ = self.spi.transfer(&mut rx, &tx).await;
        self.cs.set_high();
        rx[1]
    }

    // I2C Register Access
    async fn wr_sensor_reg8_8(&mut self, reg: u8, val: u8) {
        let _ = self.i2c.write(SENSOR_ADDR, &[reg, val]).await;
    }

    async fn rd_sensor_reg8_8(&mut self, reg: u8) -> u8 {
        let mut val = [0u8; 1];
        let _ = self.i2c.write_read(SENSOR_ADDR, &[reg], &mut val).await;
        val[0]
    }

    async fn wr_sensor_regs(&mut self, regs: &[ov2640_regs::RegisterData]) {
        for r in regs {
            if r.reg == 0xff && r.val == 0xff {
                break;
            }
            self.wr_sensor_reg8_8(r.reg, r.val).await;
        }
    }
}
