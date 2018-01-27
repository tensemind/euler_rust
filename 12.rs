fn get_factors(n: u64) -> Vec<u64> {
    (1..n + 1).into_iter().filter(|&x| n % x == 0).collect::<Vec<u64>>()
}

fn main() {
    let mut n:u64 = 0;
    let mut t_num:u64 = 0;
    loop {
        n+=1;
        t_num+=n;
        if get_factors(t_num).len() > 500 {
            println!("{}", t_num);
            break;
        }
    }
}
