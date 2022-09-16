/// @author: Leon
/// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/
/// Time Complexity:    O(`len_ms` ^ 2)
/// Space Complexity:   O(`len_ms` ^ 2)
/// Reference:
/// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/discuss/1075469/JavaC%2B%2BPython-3-Top-Down-DP-O(m2)-Clean-and-Concise
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let len_ms: usize = multipliers.len();
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; len_ms]; len_ms];
        return Self::dfs(0, 0, &nums, &multipliers, &mut memo);
    }
    fn dfs(
        idx_n: usize,
        idx_m: usize,
        nums: &Vec<i32>,
        muls: &Vec<i32>,
        memo: &mut Vec<Vec<Option<i32>>>,
    ) -> i32 {
        let len_ns: usize = nums.len();
        let len_ms: usize = muls.len();
        if idx_m == len_ms {
            return 0;
        }
        if let Some(m) = memo[idx_n][idx_m] {
            return m;
        }
        let pick_lo = Self::dfs(idx_n + 1, idx_m + 1, nums, muls, memo) + nums[idx_n] * muls[idx_m];
        let pick_hi = Self::dfs(idx_n, idx_m + 1, nums, muls, memo)
            + nums[len_ns - (idx_m - idx_n) - 1] * muls[idx_m];
        let res = std::cmp::max(pick_lo, pick_hi);
        memo[idx_n][idx_m] = Some(res);
        return res;
    }
}
