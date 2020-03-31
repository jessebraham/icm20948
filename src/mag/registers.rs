// https://invensense.tdk.com/wp-content/uploads/2016/06/DS-000189-ICM-20948-v1.3.pdf

// 12 - REGISTER MAP FOR MAGNETOMETER (Page 77)
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Registers {
    WIA2 = 0x01,
    ST1 = 0x10,
    HXL = 0x11,
    HXH = 0x12,
    HYL = 0x13,
    HYH = 0x14,
    HZL = 0x15,
    HZH = 0x16,
    ST2 = 0x18,
    CNTL2 = 0x31,
    CNTL3 = 0x32,
}
