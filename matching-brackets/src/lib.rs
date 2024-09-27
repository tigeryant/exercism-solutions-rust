pub fn brackets_are_balanced(string: &str) -> bool {
    let openers = ['[', '{', '('];
    let closers = [']', '}', ')'];

    let mut stack: Vec<char> = vec![];
    for char in string.chars() {
        if openers.contains(&char) {
            stack.push(char)
        }
        if closers.contains(&char) {
            let index = closers.iter().position(|&c| c == char).unwrap();
            if stack.pop() != Some(openers[index]) { return false; }
        }
    }
    stack.is_empty()
}


// match stack.last() {
//     None => return false,
//     Some(last) => {
//         if last == &openers[index] {
//             stack.pop();
//         } else {
//             return false;
//         }
//     }
// }

// use a 'stack' data structure and check for opening add closing brackets, braces and parentheses
// if it's opening, add it to the end of the stack
// when you find a closing char, verify that it matches the most recent element on the stack,
// if it does, remove that item (do this for as long as the stack has elements (or until the end of the iterator))
// return true if the stack's empty

// use a data structure to map pairs


/*
    match char {
        '[' => stack.push(char),
        '{' => stack.push(char),
        '(' => stack.push(char),
        ']' => {
            match stack.last() {
                None => return false,
                Some(last) => {
                    if last == &'[' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        '}' => {
            match stack.last() {
                None => return false,
                Some(last) => {
                    if last == &'{' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        ')' => {
            match stack.last() {
                None => return false,
                Some(last) => {
                    if last == &'(' {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        _ => (),
    }
*/