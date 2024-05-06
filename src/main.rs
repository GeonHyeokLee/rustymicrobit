#![no_std]
#![no_main]

use nrf52833_pac::Peripherals;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    p.P0.pin_cnf[28].write(|w| w.dir().output());

    let mut is_on = false;
    loop {
        p.P0.out.write(|w| w.pin21().bit(is_on));
        for _ in 0..200_000 {
            nop();
        }
        is_on = !is_on;
    }
}
