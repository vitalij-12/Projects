use std::{fs, io};
use crate::poke_info::*;

    fn load_items() -> Vec<Pokemon> {
    let content = fs::read_to_string("pokemon.json")
        .expect("Error: can't read pokedata");

    let pokemons: Vec<Pokemon> = serde_json::from_str(&content)
        .expect("Error: wrong json format");

    pokemons
}

    pub fn poke_find() {
        let input = input();
        let pokemons = load_items();

        if let Some(found) = pokemons.iter().find(|pokemon| pokemon.name.to_lowercase() == input.to_lowercase()){
            println!("\n\nName: {:#?}       Typ: {:#?} \nGröße: {:#?}       Gewicht: {:#?} \n\nFähigkeiten: {:#?} \n\n{:#?} \n"
            , found.name, found.poke_type, found.height, found.weight, found.abilities, found.description);
        } else {
            println!("nichts gefunden")
        }
    }

    fn input() -> String {
    // Benutzereingabe einlesen
        println!("Bitte Pokemon eingeben");

        let mut inp = String::new();
        io::stdin().read_line(&mut inp)
            .expect("Fehler beim lesen");
        inp.trim().to_string()
    }