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