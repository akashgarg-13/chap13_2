struct Fibonacci {
    current : u32,
    next : u32,
}

impl Iterator for Fibonacci{
    type Item = u32;
    fn next(&mut self) ->Option<Self::Item>{
        let new_next = self.current + self.next ; 
        self.current = self.next ; 
        self.next = new_next;
        Some(self.current)
    }
}



fn main() {
    let mut fib = Fibonacci {
        current : 1,
        next : 1,
    };
    for _ in 0..15 {
        print!("\n {:?}" ,fib.current);
        fib.next();
    }
    
}
