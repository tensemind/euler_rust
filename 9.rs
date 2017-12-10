fn main() {
    for a in 0..500 {
        for b in a+1..500 {
            let c = 1000-(a+b);
            if a*a + b*b == c*c {
                println!("{}", a*b*c);
            }
        }
    }
}
