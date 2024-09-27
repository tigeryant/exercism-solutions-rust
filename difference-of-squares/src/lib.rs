pub fn square_of_sum(n: u32) -> u32 {
    // could use an iterator from 1 to n
    // sum it
    // raise that to 2 (and return that)
    let sum: u32 = (1..=n).sum();
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // could use an iterator from 1 to n
    // map it to each of their squares
    // sum it
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
