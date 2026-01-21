#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use {esp_backtrace as _, esp_println as _};
extern crate alloc;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    image::{Image, ImageRawLE},
    primitives::Rectangle,
};
use esp_hal::{
    clock::CpuClock, 
    gpio::{Level, Output, OutputConfig}, 
    main, 
    spi::{Mode, master::{Config as SpiConfig, Spi}}, 
    time::{Duration, Instant, Rate},
    delay::Delay
};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use st7735_lcd::ST7735;

// Default App-Descriptor für den esp-idf Bootloader
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // Generator Version: 1.1.0

    // ESP32 Konfiguration und Initialisierung
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Heap-Speicher für alloc bereitstellen
    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    // Pins konfigurieren
    let sck = peripherals.GPIO9;
    let sda = peripherals.GPIO10;
    let cs   = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());
    let dc   = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
    let rst  = Output::new(peripherals.GPIO11, Level::High, OutputConfig::default());
    let _leda = Output::new(peripherals.GPIO14, Level::High, OutputConfig::default());

    let width = 160;
    let height = 128;

    // SPI-Bus initialisieren
    let spi_bus = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(2))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sck)
    .with_mosi(sda);

    // SPI exklusiv für Display nutzen
    let mut spi = ExclusiveDevice::new(
        spi_bus,
        cs,
        NoDelay
    ).unwrap();

    // Display initialisieren
    let mut display = ST7735::new(
        &mut spi,
        dc,
        rst,
        true,
        false,
        width,
        height,
    );

    let mut delay: Delay = Delay::new();
    let orientation = st7735_lcd::Orientation::LandscapeSwapped;

    display.init(&mut delay).unwrap();
    display.clear(Rgb565::MAGENTA).unwrap();
    display.set_orientation(&orientation);

    // Frames vorbereiten
    const FRAMES: &[&[u8]] = &[
        include_bytes!("../assets/rust-logo-10c/rust1.raw"),
        include_bytes!("../assets/rust-logo-10c/rust2.raw"),
        include_bytes!("../assets/rust-logo-10c/rust3.raw"),
        include_bytes!("../assets/rust-logo-10c/rust4.raw"),
        include_bytes!("../assets/rust-logo-10c/rust5.raw"),
        include_bytes!("../assets/rust-logo-10c/rust6.raw"),
        include_bytes!("../assets/rust-logo-10c/rust7.raw"),
        include_bytes!("../assets/rust-logo-10c/rust8.raw"),
        include_bytes!("../assets/rust-logo-10c/rust9.raw"),
        include_bytes!("../assets/rust-logo-10c/rust10.raw"),
        include_bytes!("../assets/rust-logo-10c/rust11.raw"),
        include_bytes!("../assets/rust-logo-10c/rust12.raw"),
        include_bytes!("../assets/rust-logo-10c/rust13.raw"),
        include_bytes!("../assets/rust-logo-10c/rust14.raw"),
        include_bytes!("../assets/rust-logo-10c/rust15.raw"),
        include_bytes!("../assets/rust-logo-10c/rust16.raw"),
        include_bytes!("../assets/rust-logo-10c/rust17.raw"),
        include_bytes!("../assets/rust-logo-10c/rust18.raw"),
        include_bytes!("../assets/rust-logo-10c/rust19.raw"),
        include_bytes!("../assets/rust-logo-10c/rust20.raw"),
        include_bytes!("../assets/rust-logo-10c/rust21.raw"),
        include_bytes!("../assets/rust-logo-10c/rust22.raw"),
        include_bytes!("../assets/rust-logo-10c/rust23.raw"),
        include_bytes!("../assets/rust-logo-10c/rust24.raw"),
        include_bytes!("../assets/rust-logo-10c/rust25.raw"),
        include_bytes!("../assets/rust-logo-10c/rust26.raw"),
        include_bytes!("../assets/rust-logo-10c/rust27.raw"),
        include_bytes!("../assets/rust-logo-10c/rust28.raw"),
        include_bytes!("../assets/rust-logo-10c/rust29.raw"),
        include_bytes!("../assets/rust-logo-10c/rust30.raw"),
        include_bytes!("../assets/rust-logo-10c/rust31.raw"),
        include_bytes!("../assets/rust-logo-10c/rust32.raw"),
        include_bytes!("../assets/rust-logo-10c/rust33.raw"),
        include_bytes!("../assets/rust-logo-10c/rust34.raw"),
        include_bytes!("../assets/rust-logo-10c/rust35.raw"),
        include_bytes!("../assets/rust-logo-10c/rust36.raw"),
    ];

    let fill = &Rectangle::new(Point::new(0, 0), Size::new(width, height));

    // Endlosschleife: Animation
    loop {
        for frame in FRAMES.iter() {
            let image_raw = ImageRawLE::<Rgb565>::new(frame, width);
            let image = Image::new(&image_raw, Point::new(0, 0));

            image.draw(&mut display).unwrap();

            // Optional: kurzer Delay pro Frame (~50ms)
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(50) {}
        }

        // Bildschirm zurücksetzen
        display.fill_solid(&fill, Rgb565::BLACK).unwrap();
    }
}

// Inspiration: https://github.com/esp-rs/esp-hal/tree/esp-hal-v~1.0/examples
