#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprint, rtt_init_print};
use cortex_m::asm::nop;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    loop {
        rprint!("Echo...\n");
        for _ in 0..100_000 {
            nop();
        }
    }
}
