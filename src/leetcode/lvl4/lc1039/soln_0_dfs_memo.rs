/// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
///
/// Time Complexity:    O(`len_n` ^ 3)
/// Space Complexity:   O(`len_n` ^ 2)
///
/// Reference:
/// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/discuss/286753/C++-with-picture/572360
/// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/discuss/286753/C%2B%2B-with-picture
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE_DATA: usize = 50;
    pub fn min_score_triangulation(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![0; len_n]; len_n];
        Self::dfs(0, len_n - 1, &nums, &mut memo)
    }

    fn dfs(lo: usize, hi: usize, nums: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[lo][hi] > 0 {
            return memo[lo][hi];
        }

        if hi - lo < 2 {
            return 0;
        }
        let mut min_score = std::i32::MAX;
        for k in lo + 1..hi {
            min_score = std::cmp::min(
                min_score,
                nums[lo] * nums[hi] * nums[k]
                    + Self::dfs(lo, k, nums, memo)
                    + Self::dfs(k, hi, nums, memo),
            );
        }
        memo[lo][hi] = min_score;
        min_score
    }
}
