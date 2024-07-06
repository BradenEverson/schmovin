use rppal::i2c::I2c;
use std::{thread, time::Duration};

const GY_521_ADDRESS: u16 = 0x68;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_XOUT_H: u8 = 0x3B;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(GY_521_ADDRESS)?;

    // Wake up the GY-521 module (exit sleep mode)
    i2c.smbus_write_byte(PWR_MGMT_1, 0)?;

    loop {
        // Read the accelerometer data
        let accel_xout_h = i2c.smbus_read_byte(ACCEL_XOUT_H)?;
        let accel_xout_l = i2c.smbus_read_byte(ACCEL_XOUT_H + 1)?;
        let accel_xout = ((accel_xout_h as i16) << 8) | (accel_xout_l as i16);

        let accel_yout_h = i2c.smbus_read_byte(ACCEL_XOUT_H + 2)?;
        let accel_yout_l = i2c.smbus_read_byte(ACCEL_XOUT_H + 3)?;
        let accel_yout = ((accel_yout_h as i16) << 8) | (accel_yout_l as i16);

        let accel_zout_h = i2c.smbus_read_byte(ACCEL_XOUT_H + 4)?;
        let accel_zout_l = i2c.smbus_read_byte(ACCEL_XOUT_H + 5)?;
        let accel_zout = ((accel_zout_h as i16) << 8) | (accel_zout_l as i16);

        println!("Accel X: {}, Accel Y: {}, Accel Z: {}", accel_xout, accel_yout, accel_zout);

        // Delay for a second
        thread::sleep(Duration::from_millis(200));
    }
}
