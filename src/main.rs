// Will only work with Range5 or higher
// Needs Range6 or higher to work with 400kHz I2C frequency

//#![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

//extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::{Config,MSIRange}};

use rand::prelude::*;
use ssd1306::{mode::displaymode::DisplayModeTrait, prelude::*, Builder as SSD1306Builder};

const BOOT_DELAY_MS: u16 = 100; 

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    // let mut rcc = dp.RCC.freeze(Config::hsi16()); 
    let mut rcc = dp.RCC.freeze(Config::msi(MSIRange::Range5)); //works only with Range5 or Range6

    let mut delay = cp.SYST.delay(rcc.clocks);

    //delay necessary for the I2C to initiate correctly and start on boot without having to reset the board

    delay.delay_ms(BOOT_DELAY_MS);

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    let scl = gpioa.pa9.into_open_drain_output();
    let sda = gpioa.pa10.into_open_drain_output();
    

    let mut i2c = dp.I2C1.i2c(sda, scl, 100.khz(), &mut rcc); 
                  
    let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();
        
    disp.init().unwrap();
    
    let mut props = disp.release();

    let mut buf = [0x00u8; 1024];

    let mut rng = SmallRng::seed_from_u64(0x0101_0303_0808_0909);

    loop {

        rng.fill_bytes(&mut buf);

        props.draw(&buf);

    }

}

