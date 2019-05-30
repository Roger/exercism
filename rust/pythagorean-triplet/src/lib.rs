pub fn find() -> Option<u32> {
    for c in 2..1000 {
        for a in 1..c {
            let b: u32 = 1000 - c - a;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Some(a * b * c)
            }
        }
    }
    None
}
