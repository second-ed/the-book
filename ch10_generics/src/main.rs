fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    dbg!(result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    dbg!(result);
    create_points();
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

fn create_points() {
    let both_int = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let mixed_types = Point { x: 't', y: 4.2 };

    dbg!(both_int);
    dbg!(both_float);
    dbg!(mixed_types);
}
