#[derive(Debug)]
enum SpecialDesign {
    Queen,
    Olympic,
    Train,
}


enum Coin {
    OneP,
    TwoP,
    FiveP,
    TenP,
    TwentyP,
    FiftyP(SpecialDesign),
    Pound,
}

fn value_in_pence(coin: &Coin) -> u8 {
    match coin {
        Coin::OneP => 1,
        Coin::TwoP => 2,
        Coin::FiveP => 5,
        Coin::TenP => 10,
        Coin::TwentyP => 20,
        Coin::FiftyP(design) => {
            println!("got special {:?} 50p", design);
            50
        },
        Coin::Pound => 100,
    }
}

fn main() {
    let change = [Coin::Pound, Coin::TenP, Coin::TwoP, Coin::FiftyP(SpecialDesign::Olympic)];

    let mut total: u16 = 0;
    for coin in change {
        let val = value_in_pence(&coin);
        println!("{val}");
        total += val as u16;
    }
    println!("total is {total}p");
    let pounds = (total as f32) / 100.0;
    println!("total in £{pounds}");

}
