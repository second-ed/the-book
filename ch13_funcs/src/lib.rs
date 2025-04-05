#[derive(PartialEq, Debug)]
enum ShoeStyle {
    Sneaker,
    Sandal,
    Boot,
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: ShoeStyle,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: ShoeStyle::Sneaker,
            },
            Shoe {
                size: 13,
                style: ShoeStyle::Sandal,
            },
            Shoe {
                size: 10,
                style: ShoeStyle::Boot,
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: ShoeStyle::Sneaker,
                },
                Shoe {
                    size: 10,
                    style: ShoeStyle::Boot,
                },
            ]
        );
    }
}
