pub fn primes_up_to(limit: u64) -> Vec<u64> {
    let mut vec: Vec<_> = (2 .. limit).collect();

    for p in 2 .. limit {
        vec.retain(|&x| x <= p || x % p != 0);
    }
    return vec;
}

fn main() {
    let prime_sum:u64 = primes_up_to(10).iter().sum();
    println!("{}", prime_sum);
}
