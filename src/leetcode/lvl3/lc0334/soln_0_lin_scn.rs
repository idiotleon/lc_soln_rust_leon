/// @author: Leon
/// https://leetcode.com/problems/increasing-triplet-subsequence/description/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let _len_ns: usize = nums.len();
        let mut first: i32 = i32::MAX;
        let mut sec: i32 = i32::MAX;
        for num in nums {
            if num <= first {
                first = num;
            } else if num <= sec {
                sec = num;
            } else {
                return true;
            }
        }
        return false;
    }
}
