// Kata: Find the odd int
// Link: https://www.codewars.com/kata/54da5a58ea159efa38000836
// Author: https://www.codewars.com/users/rbuckley

use std::collections::HashMap;

fn find_odd(arr: &[i32]) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in arr.iter() {
        let updated: &i32 = counts.get(i).unwrap_or(&0);
        counts.insert(*i, *updated + 1);
    }
    let mut res: i32 = 0;
    for (k, v) in counts.iter() {
        if v % 2 != 0 {
            res = *k;
            break;
        } else {
            res = *k;
        }
    }
    res
}

pub fn run() {
    let res = find_odd(&[1, 1, 2]);
    println!("find_odd: {}", res);
}
