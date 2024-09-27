pub fn egg_count(display_value: u32) -> usize {
    // maps over a range of 0 to 31, returning the nth bit if it's set, collecting and then summing
    let sum: u32 = (0..32).map (|n| (display_value >> n) & 1).collect::<Vec<_>>().into_iter().sum();
    sum as usize
}
