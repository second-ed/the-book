fn main() {
    capturing_references_or_moving_ownership();
    example_sort_by_key();
    processing_items_with_iterators();
}

fn capturing_references_or_moving_ownership() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    println!("\nBORROWING IMMUTABLY");
    let list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);

    let only_borrows = || println!("From closure:            {:?}", list);

    println!("Before calling closure:  {:?}", list);
    only_borrows();
    println!("After calling closure:   {:?}", list);

    println!("\nBORROWING MUTABLY");
    let mut list = vec![1, 2, 3];
    println!("before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure:   {:?}", list);
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn example_sort_by_key() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

fn processing_items_with_iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    let v2 = vec![1, 2, 4, 8];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();
    println!("total: {total}");

    let v3 = vec![0, 1, 3];
    let v3_iter = v3.iter().map(|x| x + 1);
    println!("{:?}", v3_iter);
}
