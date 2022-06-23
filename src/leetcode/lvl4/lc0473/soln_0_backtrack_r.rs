/// @author: Leon
/// https://leetcode.com/problems/matchsticks-to-square/
/// Time Complexity:    O(4 ^ `sum_all`)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/discuss/108741/Solution-with-Reference/569560
/// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/discuss/108741/Solution-with-Reference
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn make_square(matchsticks: Vec<i32>) -> bool {
        let len_n = matchsticks.len();
        let sum_all: i32 = matchsticks.iter().sum();
        if sum_all < 4 || sum_all % 4 != 0 {
            return false;
        }
        let sum_target = sum_all / 4;
        let mut used = vec![false; len_n];
        Self::can_partition(0, 0, 4, &mut used, sum_target, &matchsticks)
    }
    fn can_partition(
        idx_start: usize,
        sum_cur: i32,
        k: u8,
        used: &mut Vec<bool>,
        sum_target: i32,
        nums: &Vec<i32>,
    ) -> bool {
        let len_n = nums.len();
        if k == 0 {
            return true;
        }
        if sum_cur == sum_target {
            return Self::can_partition(0, 0, k - 1, used, sum_target, nums);
        }
        if sum_cur > sum_target {
            return false;
        }
        for idx in idx_start..len_n {
            if used[idx] {
                continue;
            }
            used[idx] = true;
            if Self::can_partition(idx + 1, sum_cur + nums[idx], k, used, sum_target, nums) {
                return true;
            }
            used[idx] = false
        }
        return false;
    }
}
