pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // we can determine the student id by mapping their first name to an integer between 0 and 11 inclusive
    // we can index the string slice based on the student id
    // could we use an iterator over the characters instead? iterate over the chars and take
    
    // to determine the student id, map the first letter of student to its integer
    // consider this:
    // Every letter has an ASCII value which can be represented in binary form. 
    // Performing the bitwise and of this value with the number 31 will give the letter’s position in the alphabet.
    // we can get the ascii as u8 by converting a char like this: char.to_ascii_lowercase() as u8
    // find a mapping between the ascii u8 and the position in the diagram (the 0 to 11 range)

    // next, we relate this first row index to the second via the length of diagram
    // the start of the second row index will be student_index + 1/2 the length of the diagram
    let start_index = ((student.as_bytes()[0] - 65) * 2) as usize;
    let row_len = diagram.len() / 2;
    
    // note that there is an off by one here because there is a whitespace character between the two lines which offsets the diagram length
    let a: usize = start_index;
    let b: usize = start_index + 1;
    let c: usize = start_index + row_len + 1;
    let d: usize = start_index + row_len + 2;

    // iterate over the chars of diagram. filter by selecting the characters at indeces a, b, c, d
    let plant_strings: Vec<&str> = diagram
        .char_indices()
        .filter(|(index, _)| index == &a || index == &b || index == &c || index == &d)
        .map(|(_, char)| {
            match char {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => panic!("invalid character in diagram")
            }
        })
        .collect::<Vec<_>>();
    plant_strings
}
