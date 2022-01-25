/// @author: Leon
/// https://leetcode.com/problems/valid-mountain-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/valid-mountain-array/discuss/194900/C%2B%2BJavaPython-Climb-Mountain
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_mountain_array(nums: Vec<i32>) -> bool {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        while lo + 1 < len_n && nums[lo] < nums[lo + 1] {
            lo += 1;
        }
        let mut hi: isize = len_n as isize - 1;
        while hi - 1 > 0 && nums[hi as usize - 1] > nums[hi as usize] {
            hi -= 1;
        }
        lo > 0 && lo as isize == hi && hi < len_n as isize - 1
    }
}
