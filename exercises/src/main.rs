use std::{f32::consts::PI, num::ParseIntError};

fn run1() {
    println!("Hello, world!");
    let x = 5;
    println!("The value of x is: {}", x);

    let x = 6;
    println!("The value of x is: {}", x);

    println!(
        "The example of shadowing is shown above. The variable x is first assigned the value 5, and then it is shadowed by a new variable with the same name that is assigned the value 6. This allows us to reuse the variable name while still keeping the original value accessible if needed."
    );

    let a: i32 = -92_000_000;
    println!("The value of a is: {}", a);

    let b: u32 = 92_000_000;
    println!("The value of b is: {}", b);

    let c: u8 = b'A';
    println!("The value of c is: {}", c);

    let d: i8 = -128;
    println!("The value of d is: {}", d);

    // c style loop
    let i: Result<i32, ParseIntError> = "32".parse();
    match i {
        Ok(num) => println!("The value of num is: {}", num),
        Err(e) => println!("Error parsing string: {}", e),
    }

    let i: Result<isize, _> = "2148468243".parse();
    match i {
        Ok(num) => println!(
            "The value of num is: {}, {}",
            num,
            std::any::type_name::<isize>()
        ),
        Err(e) => println!("Error parsing string: {}", e),
    }
    // size of isize
    println!(
        "The size of isize is: {} bytes",
        std::mem::size_of::<isize>()
    );
    let size = std::mem::size_of::<isize>();
    println!("The size of isize is: {} bits", size * 8);

    let quotient = 10.0 / 3.0;
    println!("The quotient of 10 / 3 is: {}", quotient);

    println!(
        "The example of floating point division is shown above. The division of 10.0 by 3.0 results in a floating point number, which is approximately 3.3333333333333335. This demonstrates how Rust handles floating point arithmetic, which can sometimes lead to precision issues due to the way floating point numbers are represented in memory."
    );

    // '😻'
    let heart_eyes_cat = '😻';
    let heart_eyes_cat_code_point = heart_eyes_cat as u32;
    println!(
        "The Unicode code point for heart_eyes_cat is: {}",
        heart_eyes_cat_code_point
    );
    println!("The value of heart_eyes_cat is: {}", heart_eyes_cat);

    let x = b'1';
    println!("The value of x is: {}", x);

    let tup = (b'Z', '😻', PI);
    println!("The tuple is: ({}, {}, {})", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println!("The values of x, y, z are: {}, {}, {}", x, y, z);
}

fn main() {
    run1();

    let array = [1, 2, 3, 4, 5];
    println!("The array is: {:?}", array);

    // slice of the array
    let slice = &array[1..4];
    println!("The slice of the array is: {:?}", slice);

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let idx: usize = input.trim().parse().expect("Please enter a valid number");
    println!("The index is: {}", idx);

    let element = array.get(idx).unwrap_or(&0);

    println!("The element at index {} is: {}", idx, element);
}
