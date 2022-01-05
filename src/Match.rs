#[derive(Debug)]
enum Balance {
    Small,
    Intermediate,
    Fish,
    Shark,
}

#[derive(Debug)]
enum Coin {
    Solana,
    Ethereum,
    Near,
    Bitcoin(Balance),
}

fn decimal(coin: Coin) -> u8 {
    match coin {
        Coin::Solana => {
            println!("Match Solana");
            return 1;
        }
        Coin::Ethereum => {
            println!("Match Ethereum");
            return 2;
        }
        Coin::Near => {
            println!("Match Near");
            return 3;
        }
        Coin::Bitcoin(bala) => {
            println!("I am a {:#?}", bala);
            return 4;
        }
    }
}

fn plug_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(x) => Some(x + 1),
        _ => None,
    }
}

fn main() {
    let result = decimal(Coin::Bitcoin(Balance::Shark));
    println!("{}", result);

    let five = Some(5);
    let six = plug_one(five);
    let none = plug_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}
