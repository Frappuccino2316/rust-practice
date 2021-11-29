fn main() {
    let nickel = Coin::Penny(String::from("Test"));

    // let value = value_in_cents(nickel);
    // println!("{}", value);

    if let Coin::Nickel(state) = nickel {
        println!("{}", state);
    } else {
        println!("No!!");
    }
}

enum Coin {
    Penny(String),
    Nickel(String),
    Dime(String),
    Quarter(String),
}

// fn value_in_cents(coin: Coin) -> u32 {
    // match coin {
    //     Coin::Penny => {
    //         println!("Lucky penny!");
    //         1
    //     },
    //     // Coin::Nickel => 5,
    //     // Coin::Dime => 10,
    //     // Coin::Quarter => 25,
    //     _ => {
    //         println!("other");
    //         1000
    //     },
    // }
// }
