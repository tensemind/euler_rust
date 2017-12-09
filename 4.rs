fn is_palindrome(num: i32) -> bool {
    let num_str = num.to_string();
    let rev_num_str: String = num_str.chars().rev().collect();
    return rev_num_str == num_str;
}

fn main() {
    let mut palindromes: Vec<i32> = Vec::new();
    for x in (0..1000).rev() {
        for y in (0..1000).rev() {
            if is_palindrome(x*y) {
                palindromes.push(x*y);
            }
        }
    }
    println!("{:?}", palindromes.iter().max());
}
