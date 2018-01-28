fn main() {
    let mut longest:u64 = 0;
    let mut counter:u64;
    let mut answer:u64 = 0; 
    
    for mut n in 1..1000000 {
        counter = n;
        let mut vec: Vec<u64> = Vec::new();
        
        while n > 1 {
            vec.push(n);
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
        }
        vec.push(1); // Add last number for correct length
        if vec.len() as u64 > longest {
            longest = vec.len() as u64;
            answer = counter;
        }
    }
    println!("{}", answer);
}
