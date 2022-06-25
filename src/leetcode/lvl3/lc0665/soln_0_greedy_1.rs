/// @author: Leon
/// https://leetcode.com/problems/non-decreasing-array/
/// Time Complexity:     O(`len_n`)
/// Space Complexity:    O(1)
/// Reference:
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC++-Simple-greedy-like-solution-with-explanation/241578
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC++-Simple-greedy-like-solution-with-explanation/109180
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC%2B%2B-Simple-greedy-like-solution-with-explanation
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let len_n = nums.len();
        let mut cnt_modified: i32 = 0;
        let mut nums = nums;
        for i in 1..len_n {
            if nums[i - 1] > nums[i] {
                cnt_modified += 1;
                if i == 1 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
                if cnt_modified > 1 {
                    break;
                }
            }
        }
        cnt_modified <= 1
    }
}
