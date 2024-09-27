pub fn collatz(n: u64) -> Option<u64> {
    // define an iterator
    // while result != 1
    // add to an iterator
    // match against if n is even or odd (mod by 2)
    // return a result on each of the branches

    if n == 0 {
        return None
    }

    let mut iterations: u64 = 0;
    let mut result: u64 = n;
    while result != 1 {
        iterations +=1;
        let even = result % 2 == 0;
        println!("result: {result}, even: {even}");
        match even {
            true => result /= 2,
            false => result = (result * 3) + 1
        }
    }
    Some(iterations)
}
