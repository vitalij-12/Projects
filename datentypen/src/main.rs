fn main() {

    // Primitive Datentypen: einfache, direkt gespeicherte Werte
    println!("Primitive datentypen \n");

    let signed: i32 = -100;
    let unsigned: u32 = 200;
    let float: f32 = 1.11;
    let tbool: bool = true;
    let char: char = '☢';
    let slice: &str = "slice";

    println!(
        "Integer: signed: {:?}, unsigned: {}\n\nfloat: {}\nbool: {}\nchar; {}\nstring: {}",
        signed, unsigned, float, tbool, char, slice
    );

    // Gegensatz zu primitiven Typen: komplexe / zusammengesetzte Datentypen
    println!("\nKomplexe Datentypen:");

    tupl();
    mont();
    stri();
}

fn tupl(){
    // Tuple: mehrere Werte unterschiedlicher Typen in einer festen Struktur
    let (n, a, s): (&str, u32, u32) = ("Max Muster", 1800, 1_000_000);
    println!(
        "\n\ntuples\nname: {:?}, alter: {:?}, score: {}",
        n, a, s
    );
}

fn mont(){
    // Array: feste Größe, alle Elemente vom gleichen Typ
    println!("\n\nArrays");
    let mon: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dez"
    ];
    println!("{:#?}", mon);
}

fn stri(){
    // String: heap-alloziert und veränderbar
    println!("\n\nStrings\n");
    let mut pusch = String::from("Push");
    println!("{}", pusch);

    pusch.push_str(" zwei");
    println!("{}",pusch);

    pusch.push_str(" wörter");
    println!("{}",pusch);
}
