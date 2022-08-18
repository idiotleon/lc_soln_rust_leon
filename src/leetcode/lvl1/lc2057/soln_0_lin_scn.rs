/// @author: Leon
/// https://leetcode.com/problems/smallest-index-with-equal-value/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        for (idx, num) in nums.into_iter().enumerate() {
            let idx: i32 = idx as i32;
            if idx % 10 == num {
                return idx;
            }
        }
        return -1;
    }
}
