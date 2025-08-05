#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;

use embedded_graphics::mono_font::ascii::FONT_5X8;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Text, TextStyleBuilder};
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
    let delay = Delay::new();

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

    // ---- Draw Rectangle ----
    // let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(
    //     embedded_graphics::primitives::PrimitiveStyle::with_fill(BinaryColor::On),
    // );
    let hollow_rect_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On) // Only draw the border
        .stroke_width(1) // Border thickness of 1 pixel
        .build();
    let rect = Rectangle::new(Point::new(1, 1), Size::new(6, 6)).into_styled(hollow_rect_style);
    rect.draw(&mut display).unwrap();
    display.flush().unwrap();

    delay.delay_millis(1000);
    // Uncomment to Clear the screen and buffer
    // Without this, it will draw the circle inside the previous square
    // display.clear_screen().unwrap();

    // Draw circle
    draw_circle(&mut display, Point::new(2, 2), 4).unwrap();
    display.flush().unwrap();

    delay.delay_millis(1000);

    // Just clear the buffer. it wont send request to the devices until the flush.
    display.clear_buffer();

    draw_character(&mut display, "R").unwrap();
    display.flush().unwrap();

    loop {
        delay.delay_millis(500);
    }
}

// draw hollow circle
fn draw_circle<D>(display: &mut D, position: Point, diameter: u32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let hollow_circle_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    let circle = Circle::new(position, diameter).into_styled(hollow_circle_style);
    circle.draw(display)
}

fn draw_character<D>(display: &mut D, text: &str) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let position = Point::new(4, 0);

    let text_style = TextStyleBuilder::new()
        .alignment(embedded_graphics::text::Alignment::Center)
        .baseline(embedded_graphics::text::Baseline::Top)
        .build();
    let character_style = MonoTextStyle::new(&FONT_5X8, BinaryColor::On);

    let text = Text::with_text_style(text, position, character_style, text_style);
    text.draw(display)?;
    Ok(())
}
