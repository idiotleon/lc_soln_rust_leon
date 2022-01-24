/// @author: Leon
/// https://leetcode.com/problems/intersection-of-two-arrays/
/// Time Complexity:    O(max(`len1`, `len2`))
/// Space Complexity:   O(26) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len1: usize = nums1.len();
        let len2: usize = nums2.len();
        if len1 > len2 {
            return Self::intersection(nums2, nums1);
        }
        const RANGE: u16 = 1e3 as u16 + 7;
        let mut freqs: Vec<u16> = vec![0; RANGE as usize];
        for num in nums1 {
            freqs[num as usize] += 1;
        }
        let mut ans: Vec<i32> = Vec::new();
        for num in nums2 {
            if freqs[num as usize] > 0 {
                ans.push(num);
                freqs[num as usize] = 0;
            }
        }
        ans
    }
}
