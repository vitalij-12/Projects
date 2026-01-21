use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, Rectangle, PrimitiveStyle},
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
    image::{Image, ImageRawLE}
};
use embedded_graphics_simulator::{SimulatorDisplay, Window, OutputSettingsBuilder};

fn main() -> Result<(), core::convert::Infallible> {
    // Display-Simulator initialisieren (128x160)
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(128, 160));

    // Rohdaten des Bildes laden
    let image_raw: ImageRawLE<Rgb565> = ImageRawLE::new(include_bytes!("../assets/steckbrief.raw"), 128);

    // Image-Objekt erstellen
    let image = Image::new(&image_raw, Point::new(0, 0));

    // Display löschen und Bild zeichnen
    display.clear(Rgb565::BLACK)?;
    image.draw(&mut display).unwrap();

    // Fenster-Settings und Anzeige
    let output_settings = OutputSettingsBuilder::new()
        .scale(3) // Vergrößerung
        .build();
    Window::new("Simulator: st7735", &output_settings).show_static(&display);

    Ok(())
}
