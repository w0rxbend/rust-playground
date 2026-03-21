// Kata: Create Phone Number
// Link: https://www.codewars.com/kata/525f50e3b73515a6db000b83
// Author: https://www.codewars.com/users/xDranik

fn create_phone_number(numbers: &[u8]) -> String {
    let mut buffer = String::new();

    for (i, num) in numbers.iter().enumerate() {
        if i == 0 {
            buffer.push('(');
        } else if i == 3 {
            buffer.push(')');
        } else if i == 6 {
            buffer.push('-');
        }
        buffer.push(char::from_digit((*num).into(), 10).unwrap());
    }

    buffer
}

pub fn run() {
    let result: String = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    println!("create_phone_number: {}", result);
}
