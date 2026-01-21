use std::io;

fn main() {
    loop {
        // Zahl einlesen
        let number = n_input("bitte zahl eingeben");
        
        // Operator wählen (°C oder °F)
        let operator = input("Zu °C/c oder °F/f ?").to_lowercase();
        
        // Berechnung / Umrechnung durchführen
        math(number, &operator);

        // Wiederholen-Abfrage (max. 3 Versuche)
        let mut counter = 0;
        while counter < 3 {
            let inp = input("Wiederholen? [y/n]").to_lowercase();
            
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
    };
}

fn input(prompt: &str) -> String {
    // Benutzereingabe einlesen
    println!("{}", prompt);
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)
        .expect("Fehler beim lesen");
    inp.trim().to_string()
}

fn n_input(prompt: &str) -> f64 {
    // Wiederholt fragen, bis eine gültige Zahl eingegeben wird
    loop {
        let input = input(prompt);
        match input.parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Bitte eine gültige Zahl eingeben!"),
        }
    }
}

fn math(x: f64, op: &str) { 
    // Temperatur umrechnen
    if op == "°c" {
        let mut res = x - 32.0;
        res = res * 5.0 / 9.0;
        println!("Result: {}C°", res)
    } else if op == "°f" {
        let res = x * 1.8 + 32.0;
        println!("Result: {}F°", res)
    } else {
        println!("Error: nur zu °F & °C !");
        return;
    };
}
