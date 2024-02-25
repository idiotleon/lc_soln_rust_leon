use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/contains-duplicate-ii/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/contains-duplicate-ii/discuss/61599/C%2B%2B-unordered_map-and-unordered_set
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let len_ns: usize = nums.len();
        let k: usize = k as usize;
        let mut set: HashSet<i32> = HashSet::with_capacity(len_ns);
        for (idx, &num) in nums.iter().enumerate() {
            if idx > k as usize {
                set.remove(&nums[idx - k - 1]);
            }
            if !set.insert(num) {
                return true;
            }
        }
        false
    }
}
