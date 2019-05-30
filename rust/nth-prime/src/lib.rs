fn is_prime(n: &u32) -> bool {
    let max = *n / 2 + 1;
    !(2..max).any(|x| *n % x == 0)
}

pub fn nth(n: u32) -> Option<u32> {
    (2..).filter(|x| is_prime(x))
         .take(n as usize)
         .last()
}
