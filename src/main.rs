use crate::Suit::{Clubs, Diamonds, Hearts, Spades};

#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_assignments)]
#[allow(path_statements)]
#[allow(unused_mut)]
fn main() {
    print_choice(Hearts);
    print_choice(Spades);
    print_choice(Diamonds);
    print_choice(Clubs);

    country(44);
    country(34);
    country(125);
    country(-50);
}

fn country(code: i32) {
    let country = match code {
        44 => "UK",
        34 => "Spain",
        7 => "RuZZia",
        1..=990 => "Unknown",
        _ => "Invalid"
    };
    println!("the country with code {} is {}", code, country);
}

enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades
}

fn print_choice(choice: Suit) {
    match choice {
        Hearts => println!("\u{2665}"),
        Clubs => println!("\u{2663}"),
        Diamonds => println!("\u{2666}"),
        Spades => println!("\u{2660}"),
    }
}




