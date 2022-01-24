/// @author: Leon
/// https://leetcode.com/problems/sort-array-by-parity/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        while lo < hi {
            while lo < hi && nums[lo] % 2 == 0 {
                lo += 1;
            }
            while lo < hi && nums[hi] % 2 == 1 {
                hi -= 1;
            }
            nums.swap(lo, hi);
        }
        nums
    }
}
