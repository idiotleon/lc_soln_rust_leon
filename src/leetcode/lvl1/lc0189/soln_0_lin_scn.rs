/// @author: Leon
/// https://leetcode.com/problems/rotate-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len_n: usize = nums.len();
        let k: usize = (k % len_n as i32) as usize;
        if len_n < 2 || k == 0 {
            return;
        }
        Self::reverse((0, len_n - 1), nums);
        Self::reverse((0, k - 1), nums);
        Self::reverse((k, len_n - 1), nums);
    }
    fn reverse(coord: (usize, usize), nums: &mut Vec<i32>) {
        let (mut lo, mut hi) = coord;
        while lo < hi {
            nums.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}
