/// @author: Leon
/// https://leetcode.com/problems/house-robber/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
/// Reference
/// https://leetcode.com/problems/house-robber/discuss/156523/From-good-to-great.-How-to-approach-most-of-DP-problems.
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        let mut memo: Vec<Option<i32>> = vec![None; 1 + len_n];
        Self::dfs(len_n as isize - 1, &nums, &mut memo)
    }
    fn dfs(idx: isize, nums: &Vec<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
        if idx < 0 {
            return 0;
        }
        if let Some(m) = memo[idx as usize] {
            return m;
        }
        let res = std::cmp::max(
            Self::dfs(idx - 1, nums, memo),
            Self::dfs(idx - 2, nums, memo) + nums[idx as usize],
        );
        memo[idx as usize] = Some(res);
        res
    }
}
