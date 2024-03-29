/// @author: Leon
/// https://leetcode.com/problems/remove-one-element-to-make-the-array-strictly-increasing/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let len_ns = nums.len();
        let mut cnt = 0;
        for idx in 1..len_ns {
            if nums[idx - 1] >= nums[idx] {
                cnt += 1;
                if cnt > 1 {
                    return false;
                }
                if (idx >= 2 && nums[idx - 2] >= nums[idx])
                    && (idx + 1 < len_ns && nums[idx - 1] >= nums[idx + 1])
                {
                    return false;
                }
            }
        }
        true
    }
}
