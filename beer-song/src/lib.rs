pub fn verse(n: u32) -> String {
    // return a string in the correct format, substituting the three numbers
    // include a guard clause for the edge cases (0, 1 and 2)
    
    // note: verses 99 and 98 also represent edge cases - see the exercise - reutrn hardcoded strings for these
    // in the 99 case, the 3rd substitue is 'no' - return as an edge case, there are more differences
    // in the 98 case, 'bottles' changes to 'bottle'
    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n\n")
    } else if n == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\n")
    } else if n == 2 {
        return String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n")
    }

    // n is the number of bottles

    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n\n", n, n, n - 1)
}

pub fn sing(start: u32, end: u32) -> String {
    // this function should just call verse from start to end times
    // for a particular range, call verse, passing the verse number each time
    // do we want to include end or not? use =

    let mut song = String::from("");
    if start > end {
        for i in (end..=start).rev() {
            song += &verse(i); 
        }
    } else {
        for i in start..=end {
            song += &verse(i); 
        }
    }
    song
}
