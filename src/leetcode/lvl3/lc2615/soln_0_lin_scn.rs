use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/sum-of-distances/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/sum-of-distances/solutions/3395732/ltr-rtl/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let len_ns: usize = nums.len();
        let mut num_to_cnt_l: HashMap<i32, i64> = HashMap::with_capacity(len_ns);
        let mut num_to_cnt_r: HashMap<i32, i64> = HashMap::with_capacity(len_ns);
        let mut num_to_sum_l: HashMap<i32, i64> = HashMap::with_capacity(len_ns);
        let mut num_to_sum_r: HashMap<i32, i64> = HashMap::with_capacity(len_ns);
        let mut ans: Vec<i64> = vec![0; len_ns];
        for (idx, &num) in nums.iter().enumerate() {
            ans[idx] = *num_to_cnt_l.get(&num).unwrap_or(&0) * idx as i64
                - *num_to_sum_l.get(&num).unwrap_or(&0);
            *num_to_cnt_l.entry(num).or_default() += 1;
            *num_to_sum_l.entry(num).or_default() += idx as i64;
        }
        for (idx, &num) in nums.iter().enumerate().rev() {
            ans[idx] += *num_to_sum_r.get(&num).unwrap_or(&0)
                - *num_to_cnt_r.get(&num).unwrap_or(&0) * idx as i64;
            *num_to_cnt_r.entry(num).or_default() += 1;
            *num_to_sum_r.entry(num).or_default() += idx as i64;
        }
        return ans;
    }
}
