use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/sum-of-distances/
/// Time Complexity:    O(`len_ns` ^ 2)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let len_ns: usize = nums.len();
        let num_to_ids: HashMap<i32, Vec<i64>> = {
            let mut map: HashMap<i32, Vec<i64>> = HashMap::with_capacity(len_ns);
            for (idx, num) in nums.into_iter().enumerate() {
                // this can be further optimized as a prefix sums array
                map.entry(num).or_default().push(idx as i64);
            }
            map
        };
        let mut ans: Vec<i64> = vec![0; len_ns];
        for ids in num_to_ids.into_values() {
            for &idx1 in &ids {
                let mut sum: i64 = 0;
                for &idx2 in &ids {
                    if idx1 == idx2 {
                        continue;
                    }
                    sum += (idx1 - idx2).abs();
                }
                ans[idx1 as usize] = sum;
            }
        }
        return ans;
    }
}
