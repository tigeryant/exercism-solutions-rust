pub fn nth(n: u32) -> u32 {
    // we can start building a vector of primes
    // each iteration of 2.. we try to add a prime to the vector
    // once primes_vec.len == n + 1, we return vec.last()

    let mut primes: Vec<u32> = vec![];
    'outer: for candidate in 2.. {
        // checks if candidate is prime
        for a in 2..candidate {
            if candidate % a == 0 {
                continue 'outer;
            }
        }
        primes.push(candidate);
        if primes.len() == (n + 1) as usize {
            return candidate;
        } 
    }
    2
}
