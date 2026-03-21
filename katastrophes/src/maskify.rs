// Kata: Credit Card Mask
// Link: https://www.codewars.com/kata/5412509bd436bd33920011bc
// Author: https://www.codewars.com/users/samranjbari

// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    if cc.len() > 4 {
        let pos = cc.len() - 4;
        format!("{}{}", "#".repeat(pos), &cc[pos..])
    } else {
        cc.to_string()
    }
}

pub fn run() {
    println!("maskify: {}", maskify("1"));
}
