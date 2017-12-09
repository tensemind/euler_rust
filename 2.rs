struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn main() {
    let mut answer_sum = 0;
    
    // Just generate first 100 values, should be enough7
    for i in fibonacci().take(100) {
        if i > 4000000 {
            break;
        }
        if i%2 == 0 {
            answer_sum += i;
        }
    }
    println!("{}", answer_sum);
}
