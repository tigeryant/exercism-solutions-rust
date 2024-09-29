pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    Some(
        s1.char_indices()
            .fold(0, |distance: usize, (i, c)| match c {
                c if c != s2.chars().nth(i).unwrap() => distance + 1,
                _ => distance,
            }),
    )
}
