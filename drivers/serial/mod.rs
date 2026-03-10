// drivers/serial/mod.rs
use core::fmt::{self, Write};
use x86_64::instructions::port::Port;

pub struct Serial {
    data: Port<u8>,
}

static mut SERIAL: Option<Serial> = None;

pub fn init() {
    unsafe {
        let mut serial = Serial {
            data: Port::new(0x3F8),
        };

        SERIAL = Some(serial);
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        if let Some(ref mut s) = SERIAL {
            s.data.write(byte);
        }
    }
}

pub fn write_str(s: &str) {
    for b in s.bytes() {
        write_byte(b);
    }
}

pub fn write_num(mut n: u64) {
    if n == 0 {
        write_byte(b'0');
        return;
    }

    let mut buf = [0u8; 20];
    let mut i = 0;

    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        write_byte(buf[i]);
    }
}

pub struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(s);
        Ok(())
    }
}