/// @author: Leon
/// https://leetcode.com/problems/find-pivot-index/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/find-pivot-index/discuss/109255/Short-Python-O(n)-time-O(1)-space-with-Explanation/137006
/// https://leetcode.com/problems/find-pivot-index/discuss/109255/Short-Python-O(n)-time-O(1)-space-with-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let sum_total: i32 = nums.iter().sum();
        let mut sum_cur: i32 = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            if sum_cur * 2 + num == sum_total {
                return idx as i32;
            }
            sum_cur += num;
        }
        -1
    }
}
