pub fn is_armstrong_number(num: u32) -> bool {
    // save the number of digits to a variable
    // iterate over each of the digits in num, raising them to the digit_count and mapping them to another iterator
    // sum this new iterator (in the same line as the above)
    // implicity return the result of an expression evaluating their equality

    let digit_count = num.checked_ilog10().unwrap_or(0) + 1;
    let sum_of_powers = num.to_string().chars().map(|c| {
        c.to_digit(10).unwrap().pow(digit_count)
    }).sum::<u32>();
    num == sum_of_powers
}
