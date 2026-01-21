use std::io;

fn main() {
    // Startet das Programm
    prompts();
}

fn prompts() {
    loop {
        // Zahl einlesen und Quadrat berechnen
        let number = read_n("bitte zahl angeben");
        square(number);

        // Wiederholen-Abfrage (max. 3 Versuche)
        let mut counter = 0;
        while counter < 3 {
            let inp = read_in("Wiederholen? [y/n]").to_lowercase();

            match inp.as_str() {
                "y" => break,  // Schleife wiederholen
                "n" => return, // Programm beenden
                _ => println!("[y/n]!"), // Ungültige Eingabe
            };

            counter += 1;
            while counter == 3 {
                return
            }
        }
    }
}

fn square(n: i128) {
    // Quadrat berechnen
    let res = n * n;
    println!("Result: {}² = {}", n, res)
}

fn read_in(prompt: &str) -> String {
    // Benutzereingabe einlesen
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fehler beim Lesen");
    input.trim().to_string()
}

fn read_n(prompt: &str) -> i128 {
    // Zahl einlesen und validieren
    loop {
        let input = read_in(prompt);
        match input.parse::<i128>() {
            Ok(num) => return num,
            Err(_) => println!("Bitte eine gültige Zahl eingeben!"),
        }
    }
}
