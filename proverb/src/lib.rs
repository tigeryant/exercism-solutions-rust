pub fn build_proverb(list: &[&str]) -> String {
    // we can loop for the length of the items in the list
    // each time we construct a string with the ith element and the ith + 1th element
    // finally, we construct a string with the 0th element

    // can we do this more idiomatically with an iterator?
    // create a peekable iterator
    // maybe we can use map over an iterator, and then join it at the end?
    // for the final line we manually push the string

    let mut peek_list = list.iter().peekable();
    let mut result: String = "".to_string();

    while let Some(&current) = peek_list.next() {
        if let Some(&next) = peek_list.peek() {
            result.push_str(&format!("For want of a {} the {} was lost.\n", current, next)[..]);
        } else {
            result.push_str(&format!("And all for the want of a {}.", list[0])[..]);
        }
    }
    result
}
