// Kata: Highest and Lowest
// Link: https://www.codewars.com/kata/554b4ac871d6813a03000035/train/rust
// Author: https://www.codewars.com/users/Deantwo

fn high_and_low(numbers: &str) -> String {
    let mut split: Vec<i32> = numbers
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    split.sort();
    format!("{} {}", split.last().unwrap(), split.first().unwrap())
}

pub fn run() {
    let res = high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4");
    println!("high_and_low: {}", res);
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
