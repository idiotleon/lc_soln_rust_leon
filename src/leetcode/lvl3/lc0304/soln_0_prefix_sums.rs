/// https://leetcode.com/problems/range-sum-query-2d-immutable/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/range-sum-query-2d-immutable/discuss/75350/Clean-C%2B%2B-Solution-and-Explaination-O(mn)-space-with-O(1)-time
#[allow(dead_code)]
struct NumMatrix {
    prefix_sums: Vec<Vec<i32>>,
}

#[allow(dead_code)]
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n_rows: usize = matrix.len();
        let n_cols: usize = if n_rows > 0 { matrix[0].len() } else { 0 };
        let mut prefix_sums = vec![vec![0; n_cols + 1]; n_rows + 1];
        for r in 0..n_rows {
            for c in 0..n_cols {
                prefix_sums[r + 1][c + 1] =
                    matrix[r][c] + prefix_sums[r][c + 1] + prefix_sums[r + 1][c]
                        - prefix_sums[r][c];
            }
        }
        NumMatrix { prefix_sums }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1: usize = row1 as usize;
        let row2: usize = row2 as usize;
        let col1: usize = col1 as usize;
        let col2: usize = col2 as usize;
        return self.prefix_sums[row2 + 1][col2 + 1]
            - self.prefix_sums[row2 + 1][col1]
            - self.prefix_sums[row1][col2 + 1]
            + self.prefix_sums[row1][col1];
    }
}
