/// @author: Leon
/// https://leetcode.com/problems/binary-search/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut lo: isize = 0;
        let mut hi: isize = len_n as isize;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo as usize == len_n {
            return -1;
        }
        if nums[lo as usize] == target {
            lo as i32
        } else {
            -1
        }
    }
}
