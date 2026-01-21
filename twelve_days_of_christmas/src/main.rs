//use std::char::UNICODE_VERSION;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Liedtext ausgeben
    lyrics();   
}

fn lyrics(){
    let mut verse_index  = 0;

    // Zahlenworte für die Tage
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    // Strophen des Liedes
    let strophen = [
        "A partridge in a pear tree\n", 
        "Two turtle doves and\n",
        "Three french hens\n",
        "Four calling birds\n",
        "Five golden rings\n", 
        "Six geese a-laying\n", 
        "Seven swans a-swimming\n",
        "Eight maids a-milking\n",
        "Nine ladies dancing\n",
        "Ten lords a-leaping\n",
        "Eleven pipers piping\n",
        "Twelve drummers drumming\n"
    ];

    // Erste Strophe vorbereiten
    let mut ver= format!("On the {} day of Christmas, my true love sent to me  \n", days[verse_index]);
    let mut verse = format!("{}",strophen[0]);
    let tempv = format!("{}{}",ver,strophen[0]);

    // Ausgabe der ersten Strophe langsam
    print_slow(tempv, 50);

    verse_index += 1;

    // Restliche Strophen ausgeben
    for _v in 1..12 {
        ver = format!("On the {} day of Christmas, my true love sent to me  \n", days[verse_index]);
        verse = format!("{}{}",strophen[verse_index],verse);

        let output = ver.clone() + &verse;
        print_slow(output, 50);

        verse_index += 1;
    }

    return;
}

fn print_slow(text: String, delay_ms: u64) {
    // Text zeichenweise ausgeben mit Delay
    let mut stdout = io::stdout();
    for ch in text.chars() {
        print!("{ch}");
        stdout.flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
    println!();
}
