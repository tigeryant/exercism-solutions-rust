pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size > 0 {
        let range = size.pow(2);
        let mut counter: u32 = 1;
        let mut matrix: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
        let (mut top, mut bottom) = (0, size - 1);
        let (mut left, mut right) = (0, size - 1);

        for _ in 1.. {
            for i in left..=right {
                matrix[top as usize][i as usize] = counter;
                if counter == range {
                    return matrix;
                }
                counter += 1;
            }
            top += 1;

            for i in top..=bottom {
                matrix[i as usize][right as usize] = counter;
                if counter == range {
                    return matrix;
                }
                counter += 1;
            }
            right -= 1;

            for i in (left..=right).rev() {
                matrix[bottom as usize][i as usize] = counter;
                if counter == range {
                    return matrix;
                }
                counter += 1;
            }
            bottom -= 1;

            for i in (top..=bottom).rev() {
                matrix[i as usize][left as usize] = counter;
                if counter == range {
                    return matrix;
                }
                counter += 1;
            }
            left += 1;
        }
    }
    vec![]
}
