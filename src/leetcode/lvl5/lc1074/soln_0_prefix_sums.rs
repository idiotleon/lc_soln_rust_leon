use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/
/// Time Complexity:     O(`len_rs` * (`len_cs` ^ 2))
/// Space Complexity:    O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/discuss/303750/javacpython-find-the-subarray-with-target-sum/910389
/// https://leetcode.com/problems/number-of-submatrices-that-sum-to-target/discuss/1162957/Rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let len_rs: usize = matrix.len();
        let len_cs: usize = matrix[0].len();
        let prefix_sums: Vec<Vec<i32>> = {
            let mut prefix_sums: Vec<Vec<i32>> = vec![vec![0; len_cs]; len_rs];
            for r in 0..len_rs {
                for c in 0..len_cs {
                    if c == 0 {
                        prefix_sums[r][c] = matrix[r][c];
                    } else {
                        prefix_sums[r][c] = matrix[r][c] + prefix_sums[r][c - 1];
                    }
                }
            }
            prefix_sums
        };
        let mut count: i32 = 0;
        let mut sum_to_freq = HashMap::with_capacity(len_rs * len_cs);
        for lo in 0..len_cs {
            for hi in lo..len_cs {
                sum_to_freq.clear();
                sum_to_freq.insert(0, 1);
                let mut sum = 0;
                for r in 0..len_rs {
                    sum += prefix_sums[r][hi] - if lo > 0 { prefix_sums[r][lo - 1] } else { 0 };
                    let expected = sum - target;
                    if let Some(&freq) = sum_to_freq.get(&expected) {
                        count += freq;
                    }
                    *sum_to_freq.entry(sum).or_default() += 1;
                }
            }
        }
        count
    }
}
