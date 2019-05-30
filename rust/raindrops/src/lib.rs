use std::cmp;

pub fn raindrops(n: usize) -> String {
    // Get Factors we care(from 3 to 7)
    let max = cmp::min(n / 2 + 1, 8);
    let factors: Vec<_> = (3..max)
        .filter(|x| n % x == 0)
        .chain(vec![n])
        .collect();

    let res: String = factors
        .iter()
        .map(|n| match n {
            3 => "Pling",
            5 => "Plang",
            7 => "Plong",
            _ => "",
        })
        .collect();

    // If n have not desire factors return as a string
    if res.is_empty() {
        n.to_string()
    } else {
        res
    }
}
