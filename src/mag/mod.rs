pub mod registers;

// Chip identifier for the AK09916 Magnetometer.
pub const CHIP_ID: u16 = 0x4809;

// The AK09916 Magnetometer only has one possible I2C address.
pub const ADDRESS: u8 = 0x0C;
