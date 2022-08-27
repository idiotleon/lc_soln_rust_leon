use std::collections::BTreeSet;

/// @author: Leon
/// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
/// Time Complexity:    O((`len_cs` ^ 2) * lg(`len_rs`))
/// Space Complexity:   O(`len_rs`)
/// Reference:
/// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/83599/Accepted-C++-codes-with-explanation-and-references/87953
/// https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/83599/Accepted-C%2B%2B-codes-with-explanation-and-references
/// https://youtu.be/yCQN096CwWM
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        const RANGE: i32 = 100 * 100 * 100 + 7;
        let len_rs: usize = matrix.len();
        let len_cs: usize = matrix[0].len();
        let mut ans: i32 = -RANGE;
        for lo in 0..len_cs {
            let mut row_sums: Vec<i32> = vec![0; len_rs];
            for hi in lo..len_cs {
                for idx in 0..len_rs {
                    row_sums[idx] += matrix[idx][hi];
                }
                let mut set: BTreeSet<i32> = {
                    let mut set: BTreeSet<i32> = BTreeSet::new();
                    // sanity check: in case there is only one row
                    set.insert(0);
                    set
                };
                let mut cur_sum: i32 = 0;
                for &sum in &row_sums {
                    cur_sum += sum;
                    if let Some(&num) = set.range(cur_sum - k..).next() {
                        ans = std::cmp::max(ans, cur_sum - num);
                    }
                    set.insert(cur_sum);
                }
            }
        }
        return ans;
    }
}
