use image::GenericImageView;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Kommandozeilenargumente sammeln, Programmname überspringen
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Usage: cargo run -- <input1.png> <input2.png> ...");
        return Ok(());
    }

    for input_path_str in args {
        let input_path = Path::new(&input_path_str);

        // Output-Datei ableiten: .png → .raw
        let mut output_path = input_path.to_path_buf();
        output_path.set_extension("raw");

        // PNG laden
        let img = image::open(&input_path)?;

        // Auf 160x128 skalieren (für z.B. LCD-Displays)
        let img = img.resize_exact(128, 160, image::imageops::FilterType::Nearest);

        // RAW-Datei schreiben (RGB565, little-endian)
        let mut file = File::create(&output_path)?;
        for y in 0..img.height() {
            for x in 0..img.width() {
                let pixel = img.get_pixel(x, y);
                let r = (pixel[0] >> 3) as u16;
                let g = (pixel[1] >> 2) as u16;
                let b = (pixel[2] >> 3) as u16;
                let rgb565 = (r << 11) | (g << 5) | b;
                file.write_all(&rgb565.to_le_bytes())?;
            }
        }

        // Ausgabe: Erfolgsmeldung
        println!(
            "✅ {} → {} erstellt: {}x{}, RGB565 LE",
            input_path.display(),
            output_path.display(),
            img.width(),
            img.height()
        );
    }

    Ok(())
}
