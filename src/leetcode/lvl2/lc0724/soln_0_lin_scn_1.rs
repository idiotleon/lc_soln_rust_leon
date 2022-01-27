/// @author: Leon
/// https://leetcode.com/problems/find-pivot-index/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/find-pivot-index/discuss/109255/Short-Python-O(n)-time-O(1)-space-with-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let sum_total: i32 = nums.iter().sum();
        let mut left_sum: i32 = 0;
        let mut right_sum: i32 = sum_total;
        for (idx, num) in nums.into_iter().enumerate() {
            left_sum += num;
            right_sum -= num;
            if left_sum == right_sum {
                return idx as i32;
            }
        }
        -1
    }
}
