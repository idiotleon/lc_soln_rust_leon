use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
/// Time Complexity:    O(`len_ns` * `len_s`)
/// Space Complexity:   O(`len_ns` * `len_s`)
/// Reference:
/// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/solutions/1499306/java-simple-solution-beats-100/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let len_ns: usize = nums.len();
        let len_t: usize = target.len();
        let mut sub_to_freq: HashMap<String, i32> = HashMap::with_capacity(len_ns);
        let mut ans: i32 = 0;
        for num in nums {
            let len_s: usize = num.len();
            if target.starts_with(&num) {
                let sub = &target[len_s..];
                if let Some(&freq) = sub_to_freq.get(sub) {
                    ans += freq;
                }
            }
            if target.ends_with(&num) {
                let sub = &target[..len_t - len_s];
                if let Some(&freq) = sub_to_freq.get(sub) {
                    ans += freq;
                }
            }
            *sub_to_freq.entry(num).or_default() += 1;
        }
        return ans;
    }
}
