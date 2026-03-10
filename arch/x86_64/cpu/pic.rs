// arch/x86_64/cpu/pic.rs

use x86_64::instructions::port::Port;

pub fn init() {
    unsafe {
        let mut cmd1 = Port::<u8>::new(0x20);
        let mut data1 = Port::<u8>::new(0x21);
        let mut cmd2 = Port::<u8>::new(0xA0);
        let mut data2 = Port::<u8>::new(0xA1);

        cmd1.write(0x11);
        cmd2.write(0x11);

        data1.write(0x20); // IRQs start at 32
        data2.write(0x28);

        data1.write(4);
        data2.write(2);

        data1.write(0x01);
        data2.write(0x01);

        data1.write(0);
        data2.write(0);
    }
}