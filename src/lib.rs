#![no_std]

pub mod accel;
pub mod mag;

use accel::registers::{RegisterBank, UserBank0};
use byteorder::{ByteOrder, LittleEndian};
use embedded_hal::blocking::i2c::{Write, WriteRead};

// ICM20948 Driver
pub struct Icm20948<I2C> {
    i2c: I2C,
    accel_addr: u8,
    selected_bank: RegisterBank,
}

impl<I2C, E> Icm20948<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    // Side effect-free constructor. Creates a new driver from an I2C
    // peripheral.
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            accel_addr: accel::ADDRESS,
            selected_bank: RegisterBank::_0,
        }
    }

    // -----------------------------------------------------------------------
    // device configuration & initialization

    // Enables use of the alternate I2C address.
    pub fn use_alternate_addr(&mut self) {
        self.accel_addr = accel::ADDRESS_ALT;
    }

    // Initialize the device.
    pub fn init(&mut self) -> Result<(), E> {
        let mut pwr_mgmt_1 = self.read_u8(UserBank0::PWR_MGMT_1 as u8)?;
        // Enable the temperature sensor.
        pwr_mgmt_1 &= !0x08;
        // Auto select best available clock source PLL if ready, else use
        // internal oscillator.
        pwr_mgmt_1 &= 0x01;
        self.write_u8(UserBank0::PWR_MGMT_1 as u8, pwr_mgmt_1)?;

        Ok(())
    }

    /// Resets the ICM20948 device.
    pub fn reset(&mut self) -> Result<(), E> {
        self.select_bank(RegisterBank::_0)?;
        self.write_u8(UserBank0::PWR_MGMT_1 as u8, 0x80)
    }

    // -----------------------------------------------------------------------
    // sensor data

    /// Returns device's factory-programmed and constant chip ID.
    /// This ID is device model ID and not an ICM20948's unique ID, which is
    /// stored in different register.
    pub fn id(&mut self) -> Result<u8, E> {
        self.select_bank(RegisterBank::_0)?;
        self.read_u8(UserBank0::WHO_AM_I as u8)
    }

    /// Read the built-in temperature sensor and return its value in Celsius.
    /// The raw register value needs to be transformed in order to obtain the
    /// correct units.  See "A.C. Electrical Characteristics", (Page 14).
    pub fn temperature(&mut self) -> Result<f32, E> {
        self.select_bank(RegisterBank::_0)?;

        let temp_out = self.read_u16(UserBank0::TEMP_OUT_H as u8, UserBank0::TEMP_OUT_L as u8)?;
        let temp_deg_c = ((temp_out as f32 - 21.0) / 333.87) + 21.0;

        Ok(temp_deg_c)
    }

    // -----------------------------------------------------------------------
    // internal helper functions
    
    fn select_bank(&mut self, bank: RegisterBank) -> Result<(), E> {
        if self.selected_bank != bank {
            // REG_BANK_SEL is present in all User Banks, so we don't need to
            // select one, because that would be sort of redundant.
            self.write_u8(UserBank0::REG_BANK_SEL as u8, bank as u8)?;
            self.selected_bank = bank;
        }

        Ok(())
    }

    fn read_u8(&mut self, reg: u8) -> Result<u8, E> {
        let mut data = [0, 0];

        match self.i2c.write_read(self.accel_addr, &[reg], &mut data) {
            Ok(_) => Ok(data[0]),
            Err(e) => Err(e),
        }
    }

    fn read_u16(&mut self, reg_h: u8, reg_l: u8) -> Result<u16, E> {
        let high = self.read_u8(reg_h)?;
        let low = self.read_u8(reg_l)?;

        Ok(LittleEndian::read_u16(&[low, high]))
    }

    fn write_u8(&mut self, reg: u8, data: u8) -> Result<(), E> {
        self.i2c.write(self.accel_addr, &[reg, data])
    }
}
