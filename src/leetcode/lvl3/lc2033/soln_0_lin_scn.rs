/// @author: Leon
/// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/discuss/1513317/C%2B%2B-or-concise-or-with-hint
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let nums: Vec<i32> = {
            let mut nums: Vec<i32> = Vec::with_capacity(len_rs * len_cs);
            for r in 0..len_rs {
                for c in 0..len_cs {
                    nums.push(grid[r][c]);
                }
            }
            nums.sort();
            nums
        };
        let target = nums[len_rs * len_cs / 2];
        let mut ans = 0;
        for num in nums.into_iter().rev() {
            if (num - target).abs() % x != 0 {
                return -1;
            } else {
                ans += (num - target).abs() / x;
            }
        }
        ans
    }
}
