fn sum_squares (n: i64) -> i64 {
    let mut sum = 0;
    for x in 1..n+1 {
        sum += x*x;
    }
    return sum;
}

fn square_sum (n: i64) -> i64 {
    let mut sum = 0;
    for x in 1..n+1 {
        sum += x;
    }
    return sum*sum;
}

fn main() {
    let length = 100;
    println!("{}", square_sum(length) - sum_squares(length));
}
