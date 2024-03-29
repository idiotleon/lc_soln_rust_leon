/// @author: Leon
/// https://leetcode.com/problems/maximum-sum-circular-subarray/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-sum-circular-subarray/discuss/178422/One-Pass
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let mut sum_total: i32 = 0;
        let mut max_global: i32 = nums[0];
        let mut max_local: i32 = 0;
        let mut min_global: i32 = nums[0];
        let mut min_local: i32 = 0;
        for num in nums {
            max_local = std::cmp::max(max_local + num, num);
            max_global = std::cmp::max(max_global, max_local);
            min_local = std::cmp::min(min_local + num, num);
            min_global = std::cmp::min(min_global, min_local);
            sum_total += num;
        }
        return if max_global > 0 {
            std::cmp::max(max_global, sum_total - min_global)
        } else {
            max_global
        };
    }
}
