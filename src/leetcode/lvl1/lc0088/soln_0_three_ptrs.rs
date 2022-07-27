/// https://leetcode.com/problems/merge-sorted-array/
/// Time Complexity:    O(`m` + `n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE: i32 = 1e9 as i32 + 7;
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
        let mut idx1: isize = m as isize - 1;
        let mut idx2: isize = n as isize - 1;
        let mut idx: isize = (m + n) as isize - 1;
        while idx >= 0 {
            let num1 = if idx1 < 0 {
                -Self::RANGE
            } else {
                nums1[idx1 as usize]
            };
            let num2 = if idx2 < 0 {
                -Self::RANGE
            } else {
                nums2[idx2 as usize]
            };
            nums1[idx as usize] = if num1 < num2 {
                idx2 -= 1;
                num2
            } else {
                idx1 -= 1;
                num1
            };
            idx -= 1;
        }
    }
}
