/// @author: Leon
/// https://leetcode.com/problems/move-zeroes/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len_ns: usize = nums.len();
        let mut lo: usize = 0;
        for hi in 0..len_ns {
            if nums[hi] != 0 {
                nums[lo] = nums[hi];
                lo += 1;
            }
        }
        while lo < len_ns {
            nums[lo] = 0;
            lo += 1;
        }
    }
}
