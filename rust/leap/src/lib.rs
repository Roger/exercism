pub fn is_leap_year(year: u64) -> bool {
    let is_divisible_by = |m| year % m == 0;

    // divisible by 4 except all divisible by 100 unless also divisible by 400
    is_divisible_by(4) && (!is_divisible_by(100) || is_divisible_by(400))
}
