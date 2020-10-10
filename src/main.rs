#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::pac::{CorePeripherals, Peripherals};
use hal::{prelude::*,
          rng,
          delay::Delay,
          Twim,          
        };

use cortex_m_rt::entry;

use ssd1306::{mode::displaymode::DisplayModeTrait, prelude::*, Builder, I2CDIBuilder};

const BOOT_DELAY_MS: u16 = 100; //small delay for the I2C to initiate correctly and start on boot without having to reset the board

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    // set up GPIO ports
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    // define I2C pins
    let scl = port0.p0_14.into_floating_input().degrade(); // clock
    let sda = port0.p0_16.into_floating_input().degrade(); // data

    let pins = hal::twim::Pins{
        scl: scl,
        sda: sda
    };    

    // initialize a delay provider
    let mut delay = Delay::new(core.SYST);
    
    // wait for just a moment
    delay.delay_ms(BOOT_DELAY_MS);

    // set up I2C    
    let i2c = Twim::new(p.TWIM0, pins, hal::twim::Frequency::K400);

    // set up SSD1306 display
    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_, _> = Builder::new().size(DisplaySize128x32).connect(interface).into();          
    disp.init().unwrap();
    
    // this way the whole buffer can be sent at once to the display:
    let mut props = disp.into_properties(); 

    let mut buf = [0x00u8; 512]; // empty buffer
    
    // initialize Random Numbers Generator
    let mut rng = rng::Rng::new(p.RNG);
    
    loop {
        rng.random(&mut buf); // fill the buffer with random values
        props.draw(&buf).unwrap();
    }   
    
}

