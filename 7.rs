fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
fn main () {
    let mut counter = 0;
    let mut x = 2;
    loop {
        if is_prime(x) == true {
            counter += 1;
        }
        if counter == 10001 {
            println!("{}", x);
            break;
        }
        x += 1;
    }
}
