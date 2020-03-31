pub mod registers;

// Chip identifier for the ICM20948 Accelerometer/Gyroscope.
pub const CHIP_ID: u8 = 0xEA;

// Possible I2C addresses for the ICM20948 Accelerometer/Gyroscope.
pub const ADDRESS: u8 = 0x68;
pub const ADDRESS_ALT: u8 = 0x69;
