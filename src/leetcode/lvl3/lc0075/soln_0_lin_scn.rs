/// https://leetcode.com/problems/sort-colors/
///
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let len_ns: usize = nums.len();
        if len_ns <= 1 {
            return;
        }
        let mut lo: usize = 0;
        let mut mid: usize = 0;
        let mut hi: usize = len_ns - 1;
        while mid <= hi && hi > 0 {
            if nums[mid] == 0 {
                nums.swap(lo, mid);
                lo += 1;
                mid += 1
            } else if nums[mid] == 1 {
                mid += 1;
            } else if nums[mid] == 2 {
                nums.swap(mid, hi);
                hi -= 1;
            }
        }
    }
}
