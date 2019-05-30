fn is_prime(n: &u32) -> bool {
    let max = *n / 2 + 1;
    !(2..max).any(|x| *n % x == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|x| is_prime(x))
         .take(1 + n as usize)
         .last()
         .unwrap()
}
