use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::io;

fn main(){
    loop {
        // Zahl einlesen für Fibonacci-Berechnung
        let number = n_input("bitte zahl eingeben um Fibunacci Zahl zu generieren");
        
        math(number);

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

fn math(mut i: i128) {
    // Berechnung der Fibonacci-Zahl mit BigUint für große Zahlen
    let mut x: BigUint = Zero::zero();
    let mut y: BigUint = One::one();

    let inp = i;

    if i % 2 != 0 {
        i = i + 1;
        i = i / 2;
    } else {
        i = i / 2;
    };
    
    while i > 0 {
        let next = &x + &y;
        x = next;
        let next = &y + &x;
        y = next;

        i -= 1;
    };

    // Ausgabe, abhängig davon ob Eingabe gerade oder ungerade
    if inp % 2 == 0 {
        println!("F{:?} = {:?}", inp, y);
    } else {
        println!("F{:?} = {:?}", inp, x);
    };
}

fn n_input(prompt: &str) -> i128 {
    // Wiederholt Eingabe, bis gültige Zahl
    loop {
        let input = input(prompt);
        match input.parse::<i128>() {
            Ok(num) => return num,
            Err(_) => println!("Bitte eine gültige Zahl eingeben!"),
        }
    }
}

fn input(prompt: &str) -> String {
    // Benutzereingabe einlesen
    println!("{}", prompt);
    
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)
        .expect("Fehler beim lesen");
    inp.trim().to_string()
}
