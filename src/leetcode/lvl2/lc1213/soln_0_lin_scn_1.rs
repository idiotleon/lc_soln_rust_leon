use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/intersection-of-three-sorted-arrays/
/// Time Complexity:    O(min(`len_ns1`, `len_ns2`, `len_ns3`))
/// Space Complexity:   O(min(`len_ns1`, `len_ns2`, `len_ns3`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn arrays_intersection(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let len_ns1: usize = nums1.len();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        let set3: HashSet<i32> = nums3.into_iter().collect();
        let mut ans: Vec<i32> = Vec::with_capacity(len_ns1);
        for num in nums1 {
            if set2.contains(&num) && set3.contains(&num) {
                ans.push(num);
            }
        }
        return ans;
    }
}
