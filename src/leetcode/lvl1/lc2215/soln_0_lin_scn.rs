use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/find-the-difference-of-two-arrays/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let len_n: usize = nums1.len();
        let set1: HashSet<i32> = nums1.iter().cloned().collect();
        let set2: HashSet<i32> = nums2.iter().cloned().collect();
        let mut ans: Vec<HashSet<i32>> = vec![HashSet::with_capacity(len_n); 2];
        for num in nums1 {
            if !set2.contains(&num) {
                ans[0].insert(num);
            }
        }
        for num in nums2 {
            if !set1.contains(&num) {
                ans[1].insert(num);
            }
        }
        ans.into_iter().map(|hs| hs.into_iter().collect()).collect()
    }
}
