#[derive(Copy, Clone)]
pub struct Collatz {
   pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 || self.v == 1 {
            return None;
        }
        
        let val = self.v;
        self.v = if val % 2 == 0 {
            val / 2
        } else {
            val * 3 + 1
        };
        
        Some(Collatz { v: val })
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    Collatz::new(n).count()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(12), 9);
        assert_eq!(collatz(0), 0);
        assert_eq!(collatz(16), 4);
        assert_eq!(collatz(4), 2);
        assert_eq!(collatz(5), 5);
        assert_eq!(collatz(6), 8);
        assert_eq!(collatz(7), 16);
        assert_eq!(collatz(8), 3);
    }
}
