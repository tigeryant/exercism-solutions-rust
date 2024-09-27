pub fn is_leap_year(year: u64) -> bool {
    // maybe nested if statements/guard clauses would work here?
    if year % 4 == 0 {
        if year % 100 == 0 {
            return year % 400 == 0
        }
        return true;
    }
    false
}
