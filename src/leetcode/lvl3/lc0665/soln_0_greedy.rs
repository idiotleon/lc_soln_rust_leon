/// @author: Leon
/// https://leetcode.com/problems/non-decreasing-array/
/// Time Complexity:     O(`len_n`)
/// Space Complexity:    O(1)
/// Reference:
/// https://leetcode.com/problems/non-decreasing-array/discuss/106826/JavaC%2B%2B-Simple-greedy-like-solution-with-explanation
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let len_n = nums.len();
        let mut cnt: i32 = 0;
        let mut nums = nums;
        for i in 1..len_n {
            if nums[i - 1] > nums[i] {
                cnt += 1;
                if i as i32 - 2 < 0 || nums[i - 2] <= nums[i] {
                    nums[i - 1] = nums[i];
                } else {
                    nums[i] = nums[i - 1];
                }
                if cnt > 1 {
                    break;
                }
            }
        }
        cnt <= 1
    }
}
