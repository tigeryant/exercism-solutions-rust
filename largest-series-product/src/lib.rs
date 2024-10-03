#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    // handle edge cases
    // handle with match statements?
    // return error spantoolong if span > string_digits.len
    // if NOT all is_numeric return invaliddigit (with the specific character)
    // - could include this by filtering in the main iterator and returning if returning true in closure body

    // optimisations:
    // use one big match statement instead?
    // return as soon as you find a non numeric char
    // look into nested mapping

    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    // move this find into the block below
    if let Some(not_numeric) = string_digits
        .chars()
        .find(|c| !c.is_numeric()) {
            return Err(Error::InvalidDigit(not_numeric));
        };

    Ok(string_digits
        .chars()
        .collect::<Vec<char>>()
        .windows(span)
        .map(|window| {
            // look up nested mapping in rust in chatGPT
            window
                .iter()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .product::<u64>()
        })
        .max()
        .unwrap())
}
