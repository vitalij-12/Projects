use std::io;

fn prompt() {
    loop {
        // Erste Zahl einlesen
        let number = n_input("bitte Zahl eingeben");

        // Operator wählen
        let operator = r_input("bitte rechenweg angeben [+, -, /, *]");

        // Zweite Zahl einlesen
        let number1 = n_input("bitte 2te Zahl eingeben");

        // Berechnung durchführen
        math(number, number1, &operator);
        
        // Wiederholen-Abfrage (max. 3 Versuche)
        let mut counter = 0;
        while counter < 3 {
            let inp = r_input("Wiederholen? [y/n]").to_lowercase();
            
            match inp.as_str() {
                "y" => break,  // Schleife wiederholen
                "n" => return, // Programm beenden
                _ => println!("[y/n]!"), // Ungültige Eingabe
            };

            counter += 1;
            while counter == 3 {
                return
            }
        };
    }
}

fn r_input(prompt: &str) -> String {
    // Benutzereingabe einlesen
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Fehler beim lesen");
    input.trim().to_string()
}

fn n_input(prompt: &str) -> f64 {
    // Zahl einlesen und validieren
    loop {
        let input = r_input(prompt);
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Bitte eine gültige Zahl eingeben!"),
        }
    }
}

fn math(x: f64, y: f64, op: &str) {
    // Berechnung basierend auf dem Operator
    let basic = match op {
        "+" => Ok(x + y),
        "-" => Ok(x - y),
        "/" => Ok(x / y),
        "*" => Ok(x * y),
        "%" => Ok(x % y),
        _ => Err("Error: ungültiger operator"),
    };

    match basic {
        Ok(value) => println!("Ergebnis: {}", value),
        Err(err) => println!("{}", err),
    };
}

fn main() {
    prompt();
}
