#[derive(Debug)]
enum SpecialDesign {
    Queen,
    Olympic,
    Train,
}

enum Coin {
    Chocolate,
    OneP,
    TwoP,
    FiveP,
    TenP,
    TwentyP,
    FiftyP(SpecialDesign),
    Pound,
}

fn value_in_pence(item: &Option<Coin>) -> Option<u8> {
    match item {
        Some(coin) => match coin {
            Coin::Chocolate => Some(0),
            Coin::OneP => Some(1),
            Coin::TwoP => Some(2),
            Coin::FiveP => Some(5),
            Coin::TenP => Some(10),
            Coin::TwentyP => Some(20),
            Coin::FiftyP(design) => {
                println!("got special {:?} 50p", design);
                Some(50)
            }
            Coin::Pound => Some(100),
        },
        _ => None,
    }
}

fn main() {
    let change: [Option<Coin>; 6] = [
        Some(Coin::Pound),
        Some(Coin::TenP),
        Some(Coin::Chocolate),
        Some(Coin::TwoP),
        None,
        Some(Coin::FiftyP(SpecialDesign::Olympic)),
    ];

    let mut total: u16 = 0;
    for coin in change {
        if let Some(Coin::Chocolate) = coin {
            println!("Whoops guess that wasn't a coin");
        } else {
            let val = value_in_pence(&coin).unwrap_or(0);
            println!("{val}");
            total += val as u16;
        }
    }
    println!("total is {total}p");
    let pounds = (total as f32) / 100.0;
    println!("total in £{pounds}");
}
