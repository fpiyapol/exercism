pub fn is_leap_year(year: u64) -> bool {
    let is_leap = if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
        true
    } else {
        false
    };

    is_leap
}
