use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/find-anagram-mappings/
/// Note:
/// this is a wrong solution.
/// it does not take duplicates into consideration.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums1.len();
        let num_to_idx: HashMap<i32, usize> = {
            let mut map: HashMap<i32, usize> = HashMap::new();
            for (idx, num) in nums2.into_iter().enumerate() {
                map.insert(num, idx);
            }
            map
        };
        let mut ans: Vec<i32> = vec![0; len_n];
        for (idx1, num) in nums1.into_iter().enumerate() {
            if let Some(&idx2) = num_to_idx.get(&num) {
                ans[idx1] = idx2 as i32;
            }
        }
        ans
    }
}
