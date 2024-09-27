pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
    // turn the string into characters
    // create a new temp variable
    // from the string length down to 0, add chars at that index to the temp variable
    // make the temp variable a string
    
    // update: use the iterators and iterator consumers:
    // chars, rev, collect

    input.chars().rev().collect()
}
