use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/continuous-subarray-sum/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/continuous-subarray-sum/discuss/99499/Java-O(n)-time-O(k)-space
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let _len_n: usize = nums.len();
        let mut prefix_sums_to_idx: HashMap<i32, isize> = {
            let mut map = HashMap::new();
            map.insert(0, -1);
            map
        };
        let mut sum: i32 = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            sum += num;
            if k != 0 {
                sum %= k;
            }
            let idx_prev = *prefix_sums_to_idx.entry(sum).or_insert(idx as isize);
            if idx as isize - idx_prev > 1 {
                return true;
            }
        }
        false
    }
}
