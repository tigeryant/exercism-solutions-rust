pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    // collect into an Option::<usize>
    // try and return this implicitly and use Option<usize>
    if s1.len() != s2.len() {
        return None;
    }

    let distance: usize = s1
        .char_indices()
        .fold(0, |distance: usize, (i, c)| match c {
            c if c != s2.chars().nth(i).unwrap() => distance + 1,
            _ => distance,
        });
    Some(distance)
}
