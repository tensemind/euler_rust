fn main() {
    let mut s = 0;
    
    for x in 1..1000 {
        if x%3 == 0 || x%5 == 0 {
            s += x;
        }
    }
    println!("{}", s);
}
