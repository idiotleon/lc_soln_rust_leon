/// @author: Leon
/// https://leetcode.com/problems/rotate-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len_n: usize = nums.len();
        let k: usize = {
            let mut k: usize = k as usize;
            k %= len_n;
            k
        };
        if len_n < 2 || k == 0 {
            return;
        }
        Self::reverse(0, len_n, nums);
        Self::reverse(0, k, nums);
        Self::reverse(k, len_n, nums);
    }
    fn reverse(lo: usize, hi: usize, nums: &mut Vec<i32>) {
        let mut lo: usize = lo;
        let mut hi: usize = hi - 1;
        while lo < hi {
            let tmp = nums[lo];
            nums[lo] = nums[hi];
            nums[hi] = tmp;
            lo += 1;
            hi -= 1;
        }
    }
}
