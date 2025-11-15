use core::ptr::{read_volatile, write_volatile};

pub fn ReadReg8(addr: usize) -> u8 {
    unsafe {
        return read_volatile(addr as *const u8);
    }
}

pub fn WriteReg8(addr: usize, newValue: u8){
    unsafe {
        write_volatile(addr as *mut u8, newValue);
    }
}

pub fn WriteBit(addr: usize, bitToWrite: u8, bitValue: bool){
    let mut reg = ReadReg8(addr);
    let mask = 1u8 << bitToWrite;
    if bitValue{
        reg |= mask;
    }
    else {
        reg &= !mask;
    }
    WriteReg8(addr, reg);
}

pub fn ReadBit(addr: usize, bitToRead: u8) -> bool{
    let value = ReadReg8(addr);
    return ((value >> bitToRead) & 1) != 0;
}

pub fn ToggleBit(addr: usize, bitToWrite: u8){
    let bitValue = ReadBit(addr, bitToWrite);
    WriteBit(addr, bitToWrite, !bitValue);
}

pub fn write_mask(addr: usize, mask: u8, value: u8) {
    let mut reg = ReadReg8(addr);
    reg = (reg & !mask) | (value & mask);
    WriteReg8(addr, reg);
}