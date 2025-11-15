use super::hal_core::{ReadBit, ReadReg8, ToggleBit, WriteBit, WriteReg8};
use super::registers::{DdrReg, PinReg, PortReg};

pub fn PinMode(ddrReg: DdrReg, pin: u8, isOutput: bool) {
    WriteBit(ddrReg as usize, pin, isOutput);
}

pub fn DigitalWrite(portReg: PortReg, pin: u8, value: bool) {
    WriteBit(portReg as usize, pin, value);
}

pub fn DigitalRead(portReg: PortReg, pin: u8) -> bool {
    return ReadBit(portReg as usize, pin);
}

pub fn Toggle(port: PortReg, pin: u8) {
    return ToggleBit(port as usize, pin);
}

pub fn enable_pullup(port: PortReg, pin: u8, enabled: bool) {
    WriteBit(port as usize, pin, enabled);
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