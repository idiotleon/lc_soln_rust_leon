/// @author: Leon
/// https://leetcode.com/problems/count-pairs-whose-sum-is-less-than-target/
/// Time Complexity:    O(`len_ns` ^ 2)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut cnt: i32 = 0;
        for lo in 0..len_ns - 1 {
            for hi in lo + 1..len_ns {
                let sum = nums[lo] + nums[hi];
                if sum < target {
                    cnt += 1;
                }
            }
        }
        return cnt;
    }
}
