pub fn square(s: u32) -> u64 {
    // find a relationship between the index and the number of grains

    // the only exception is if s = 1. in that case we return 1
    // think about casting between integer types
    // formula is to use powers base 2
    match s {
        0 => panic!("0 is an invalid square"),
        too_large if too_large > 64 => panic!("squares greater than 64 are invalid"),
        _ => 2u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    // we could use an iterator over the squares, calling square within fold and accumulating
    (1..=64).fold(0, |acc, s| square(s) + acc)
}
