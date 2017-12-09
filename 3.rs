fn main() {
    prime_factors(600851475143);
}

fn prime_factors(mut n: i64) {
    let mut factors: Vec<i64> = Vec::new();
    let mut d = 2;
    
    while n > 1 {
        while n%d == 0 {
            factors.push(d);
            n = n / d;
        }
        d = d + 1;
    }
    println!("{:?}", factors.iter().max());
}
