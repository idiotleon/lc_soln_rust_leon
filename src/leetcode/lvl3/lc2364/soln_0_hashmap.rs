use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/count-number-of-bad-pairs/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/count-number-of-bad-pairs/discuss/2388090/C%2B%2B-Python3-Hashmap-O(n)\
/// https://leetcode.com/problems/count-number-of-bad-pairs/discuss/2388183/Invalid-Total-Valid-or-n*(n-1)2-Valid-oror-(C%2B%2BJava)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let len_ns: usize = nums.len();
        let mut cnt: i64 = 0;
        let mut num_to_freq: HashMap<i32, i64> = HashMap::with_capacity(len_ns);
        for (idx, num) in nums.into_iter().enumerate() {
            let diff: i32 = num - idx as i32;
            cnt += idx as i64
                - if let Some(&freq) = num_to_freq.get(&diff) {
                    freq
                } else {
                    0
                };
            *num_to_freq.entry(diff).or_default() += 1;
        }
        return cnt;
    }
}
