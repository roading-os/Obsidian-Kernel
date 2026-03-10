// drivers/serial/uart16550.rs
use x86_64::instructions::port::Port;
use super::SerialDevice;

pub struct Uart16550 {
    data: Port<u8>,
    interrupt_enable: Port<u8>,
    fifo_control: Port<u8>,
    line_control: Port<u8>,
    modem_control: Port<u8>,
}

impl Uart16550 {
    pub const fn new(base: u16) -> Self {
        Self {
            data: Port::new(base),
            interrupt_enable: Port::new(base + 1),
            fifo_control: Port::new(base + 2),
            line_control: Port::new(base + 3),
            modem_control: Port::new(base + 4),
        }
    }

    pub unsafe fn init(&mut self) {
        self.interrupt_enable.write(0x00);
        self.line_control.write(0x80); // DLAB
        self.data.write(0x03);         // divisor low
        self.interrupt_enable.write(0x00);
        self.line_control.write(0x03); // 8N1
        self.fifo_control.write(0xC7);
        self.modem_control.write(0x0B);
    }
}

impl SerialDevice for Uart16550 {
    fn write_byte(&mut self, byte: u8) {
        unsafe {
            self.data.write(byte);
        }
    }
}