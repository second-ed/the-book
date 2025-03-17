fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    dbg!(result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    dbg!(result);
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
