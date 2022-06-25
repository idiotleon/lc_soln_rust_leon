/// @author: Leon
/// https://leetcode.com/problems/non-decreasing-array/
/// Time Complexity:     O(`len_n`)
/// Space Complexity:    O(1)
/// Reference:
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC++-Simple-greedy-like-solution-with-explanation/109181
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC++-Simple-greedy-like-solution-with-explanation/241578
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC++-Simple-greedy-like-solution-with-explanation/109180
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC%2B%2B-Simple-greedy-like-solution-with-explanation
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let len_n = nums.len();
        let mut was_modified: bool = false;
        for idx in 1..len_n {
            if nums[idx - 1] > nums[idx] {
                if was_modified {
                    return false;
                } else {
                    was_modified = true;
                    if idx >= 2 && nums[idx] < nums[idx - 2] {
                        nums[idx] = nums[idx - 1];
                    }
                }
            }
        }
        true
    }
}
