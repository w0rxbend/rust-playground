// Kata: Beginner Series #3 Sum of Numbers
// Link: https://www.codewars.com/kata/55f2b110f61eb01779000053
// Author: https://www.codewars.com/users/Vortus

fn get_sum(a: i64, b: i64) -> i64 {
    (a.min(b)..=a.max(b)).sum()
}

pub fn run() {
    println!("get_sum: {}", get_sum(1, 1));
}
