pub fn raindrops(n: u32) -> String {
    // map over the elements
    // in each closure, return a string or empty string
    // collect the result
    // return n if it's empty
    // join it with ("")

    let mut result: Vec<String> = vec![];
    let _ = (3..=7)
        .step_by(2)
        .map(|d| {
            if n % d == 0 {
                match d {
                    3 => result.push("Pling".to_string()),
                    5 => result.push("Plang".to_string()),
                    7 => result.push("Plong".to_string()),
                    _ => result.push("".to_string()),
                };
            }
            "".to_string()
        })
        .collect::<Vec<String>>();
    if result.is_empty() {
        return n.to_string();
    }
    result.join("")
}
