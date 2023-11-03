/// @author: Leon
/// https://leetcode.com/problems/apply-operations-to-an-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let mut idx: usize = 0;
        while idx < len_ns - 1 {
            if nums[idx] == nums[idx + 1] {
                nums[idx] *= 2;
                nums[idx + 1] = 0;
                idx += 2;
            } else {
                idx += 1;
            }
        }
        Self::shift(&mut nums);
        return nums;
    }
    fn shift(nums: &mut Vec<i32>) {
        let len_ns: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_ns {
            if nums[hi] != 0 {
                nums[lo] = nums[hi];
                lo += 1;
            }
            hi += 1;
        }
        while lo < len_ns {
            nums[lo] = 0;
            lo += 1;
        }
    }
}
