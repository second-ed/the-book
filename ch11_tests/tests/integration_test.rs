use ch11_tests;

mod common;

#[test]
fn add_two() {
    common::setup();
    assert_eq!(4, ch11_tests::add(2, 2));
}
