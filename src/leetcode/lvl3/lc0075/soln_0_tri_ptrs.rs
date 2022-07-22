/// @author: Leon
/// https://leetcode.com/problems/sort-colors/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let len_ns: isize = nums.len() as isize;
        let mut lo: isize = 0;
        let mut hi: isize = len_ns - 1;
        while lo < len_ns && nums[lo as usize] == 0 {
            lo += 1;
        }
        while hi >= 0 && nums[hi as usize] == 2 {
            hi -= 1;
        }
        let mut mid: isize = lo;
        while mid <= hi {
            if nums[mid as usize] == 0 {
                nums.swap(lo as usize, mid as usize);
                lo += 1;
                mid += 1;
            } else if nums[mid as usize] == 2 {
                nums.swap(mid as usize, hi as usize);
                hi -= 1;
            } else {
                mid += 1;
            }
        }
    }
}
