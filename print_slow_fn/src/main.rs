use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn print_slow(text: &str, delay_ms: u64) {
    let mut stdout = io::stdout();

    for ch in text.chars() {
        print!("{ch}");
        stdout.flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
    println!();
}

fn main(){
    print_slow("Dieser text sollte langsam sein", 50)
}