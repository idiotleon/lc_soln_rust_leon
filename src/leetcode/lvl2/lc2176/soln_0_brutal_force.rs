use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/
/// Time Complexity:    O(`_len_n` ^ 2)
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/discuss/1783416/JavaPython-3-Traverse-indices-with-same-values.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let _len_n: usize = nums.len();
        let num_to_indices: HashMap<i32, Vec<usize>> = {
            let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
            for (idx, num) in nums.into_iter().enumerate() {
                map.entry(num).or_default().push(idx);
            }
            map
        };
        let mut cnt: i32 = 0;
        for (_, indices) in num_to_indices.iter() {
            let len_ids: usize = indices.len();
            for lo in 0..len_ids - 1 {
                for hi in lo + 1..len_ids {
                    if (indices[lo] * indices[hi]) % k as usize == 0 {
                        cnt += 1;
                    }
                }
            }
        }
        cnt
    }
}
