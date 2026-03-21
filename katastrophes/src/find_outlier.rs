// Kata: Find The Parity Outlier
// Link: https://www.codewars.com/kata/5526fc09a1bbd946250002dc
// Author: https://www.codewars.com/users/obnounce

fn find_outlier(values: &[i32]) -> i32 {
    let (mut evens, mut odds): (Vec<i32>, Vec<i32>) = (vec![], vec![]);
    for &i in values.iter() {
        if i % 2 == 0 {
            evens.push(i);
        } else {
            odds.push(i);
        }
    }

    if evens.len() == 1 {
        *(evens.first().unwrap())
    } else {
        *(odds.first().unwrap())
    }
}

pub fn run() {
    let res = find_outlier(&[1, 1, 2]);
    println!("find_outlier: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}
