pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // can we use iterators?
    // chaining/nesting iterators
    // for each factor in factors:
    // create a list, where elements are the result of the factor added to itself (or multiplied by the natural numbers). stop iterating when we get to the limit
    // only add to the multiples list if that number isn't there already
    // return the sum use sum()

    // handle the 0 edgecase:
    // remove all 0s from the factors
    // if factors is_empty, return 0

    // can we use fold? chain? map? map_flatten?
    // we want to take a single factor and create an iterator from it

    let factors = factors.iter().filter(|f| **f != 0).collect::<Vec<&u32>>();
    if factors.is_empty() {
        return 0;
    }

    let mut multiples: Vec<u32> = vec![];
    for factor in factors {
        for i in 1.. {
            let product = i * factor;
            if product >= limit {
                break
            }
            if !multiples.contains(&product) {
                multiples.push(product);
            }
        }
    }
    multiples.iter().sum()
}
