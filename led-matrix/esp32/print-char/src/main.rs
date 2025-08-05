#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::master::Spi;
use esp_hal::spi::Mode as SpiMode;
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use max7219_display::led_matrix::display::SingleMatrix;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(_spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(10))
            .with_mode(SpiMode::_0),
    )
    .unwrap()
    //CLK
    .with_sck(peripherals.GPIO18)
    //DIN
    .with_mosi(peripherals.GPIO23);
    let cs = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());

    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    // Create a display instance for a single 8x8 LED matrix (not daisy-chained)
    let mut display = SingleMatrix::from_spi(spi_dev).expect("display count 1 should not panic");

    // Set brightness (intensity level) of the only device at index 0
    display.driver().set_intensity(0, 1).unwrap();

    let delay = Delay::new();

    // Loop through the "Rust" text and print characters one by one
    loop {
        let text = "Rust";

        for character in text.chars() {
            // Draw the current character in device index 0
            display.draw_char(0, character).unwrap();

            // Wait for 500 ms before clearing
            delay.delay_millis(500);

            // Clear that device
            display.clear(0).unwrap();
        }
        delay.delay_millis(500);
    }
}
