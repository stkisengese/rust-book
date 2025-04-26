#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> { 
        if self.v == 1 || self.v == 0 {
            return None;
        }
        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = 3 * self.v + 1;
        }
        Some(self.v)
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
        assert_eq!(collatz(1), 0);
        assert_eq!(collatz(4), 2);
        assert_eq!(collatz(5), 5);
        assert_eq!(collatz(6), 8);
        assert_eq!(collatz(7), 16);
        assert_eq!(collatz(8), 3);
    }
}
