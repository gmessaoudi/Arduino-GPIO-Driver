use std::ptr::{read_volatile, write_volatile};

#[derive(Copy, Clone)]
pub enum PinReg{
    PINB=0x23,
    PINC=0x26,
    PIND=0x29,
}

#[derive(Copy, Clone)]
pub enum DdrReg{
    DDRB=0x24,
    DDRC=0x27,
    DDRD=0x2A,
}

#[derive(Copy, Clone)]
pub enum PortReg{
    PORTB=0x25,
    PORTC=0x28,
    PORTD=0x2B,
}

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

pub fn PinMode(ddrReg: DdrReg, pin: u8, isOutput: bool){
    WriteBit(ddrReg as usize, pin, isOutput);
}

pub fn DigitalWrite(portReg: PortReg, pin: u8, value: bool){
    WriteBit(portReg as usize, pin, value);
}

pub fn DigitalRead(portReg: PortReg, pin: u8) -> bool{
    return ReadBit(portReg as usize, pin);
}

pub fn Toggle(port: PortReg, pin: u8) {
    return ToggleBit(port as usize, pin);
}

pub fn enable_pullup(port: PortReg, pin: u8, enabled: bool) {
    WriteBit(port as usize, pin, enabled);
}

pub fn write_mask(addr: usize, mask: u8, value: u8) {
    let mut reg = ReadReg8(addr);
    reg = (reg & !mask) | (value & mask);
    WriteReg8(addr, reg);
}

pub fn write_port(port: PortReg, value: u8) {
    WriteReg8(port as usize, value);
}

pub fn read_port(pin: PinReg) -> u8 {
    return ReadReg8(pin as usize);
}

pub struct GpioPin {
    ddr: usize,
    port: usize,
    pin: usize,
    bit: u8,
}

impl GpioPin {
    pub fn new(ddr: DdrReg, port: PortReg, pin: PinReg, bit: u8) -> Self {
        Self {
            ddr: ddr as usize,
            port: port as usize,
            pin: pin as usize,
            bit,
        }
    }

    pub fn set_mode(&self, output: bool) {
        WriteBit(self.ddr, self.bit, output);
    }

    pub fn write(&self, value: bool) {
        WriteBit(self.port, self.bit, value);
    }

    pub fn read(&self) -> bool {
        ReadBit(self.pin, self.bit)
    }

    pub fn toggle(&self) {
        ToggleBit(self.port, self.bit)
    }
}
