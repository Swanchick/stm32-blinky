#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32l4xx_hal as hal;

use defmt_rtt as _;
use panic_probe as _;
use rt::entry;

use defmt::*;

use cortex_m::Peripherals as CpuPeripherals;
use hal::{delay::Delay, pac::Peripherals as McuPeripherals, prelude::*};

#[entry]
fn main() -> ! {
    let cpu_peripherals = CpuPeripherals::take().unwrap();
    let mcu_peripherals = McuPeripherals::take().unwrap();

    let mut flash = mcu_peripherals.FLASH.constrain();
    let mut rcc = mcu_peripherals.RCC.constrain();
    let mut pwr = mcu_peripherals.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);

    let mut delay = Delay::new(cpu_peripherals.SYST, clocks);

    let mut gpioa = mcu_peripherals.GPIOA.split(&mut rcc.ahb2);

    let mut led = gpioa
        .pa8
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    loop {
        delay.delay_ms(1000u32);

        debug!("Hello World");
        info!("Hi!");
        led.toggle();
    }
}
