// drivers/timer/pit.rs
use x86_64::instructions::port::Port;

pub fn init() {
    let divisor: u16 = 1193182 / 100; // 100 Hz

    unsafe {
        let mut cmd = Port::<u8>::new(0x43);
        let mut data = Port::<u8>::new(0x40);

        cmd.write(0x36);

        data.write((divisor & 0xFF) as u8);
        data.write((divisor >> 8) as u8);
    }
}