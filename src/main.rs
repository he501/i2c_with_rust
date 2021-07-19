use rppal::i2c::I2c;

const ADDR_I2C: u16 = 0x2F;
const VALUE: u8 = 5;

fn acquire() -> Result<u8, rppal::i2c::Error> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(ADDR_I2C)?;
    i2c.write(&[VALUE])?;
    Ok(0)
}

fn main() {
    let _result = acquire();
}
