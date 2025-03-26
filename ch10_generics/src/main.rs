use ch10_generics::use_traits;
use std::fmt::Display;

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    dbg!(result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    dbg!(result);
    create_points();
    use_traits();

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let longest_str = longest(string1.as_str(), string2);
    dbg!(longest_str);

    let _ = longest_with_an_announcement("abcd", "xyz", 2);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn dist_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn create_points() {
    let both_int = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let mixed_types = Point { x: 't', y: 4.2 };

    dbg!(&both_int);
    dbg!(&both_float);
    dbg!(&mixed_types);
    dbg!(&both_int.x());
    dbg!(&both_float.dist_from_origin());
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
