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
        let mut hi: isize = len_n as isize - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            }
            if nums[mid as usize] > target {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        -1
    }
}
