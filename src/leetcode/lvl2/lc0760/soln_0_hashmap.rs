use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/find-anagram-mappings/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums1.len();
        let mut num_to_indices: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, num) in nums2.into_iter().enumerate() {
            num_to_indices.entry(num).or_default().push(idx);
        }
        let mut ans: Vec<i32> = vec![0; len_n];
        for (idx, num) in nums1.into_iter().enumerate() {
            if let Some(indices) = num_to_indices.get_mut(&num) {
                ans[idx] = indices.pop().unwrap() as i32;
            };
        }
        ans
    }
}
