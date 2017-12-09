fn main() {
    let mut smallest_num = 1;
    loop {
        let mut done = false;
        for x in 1..21 {
            if smallest_num % x != 0 {
                done = true;
                break;
            }
        }
        if done == false {
            println!("{}", smallest_num);
            break;
        }
        smallest_num += 1;
    }
}
