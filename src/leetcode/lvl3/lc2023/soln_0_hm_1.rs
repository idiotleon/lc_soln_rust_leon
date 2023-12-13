use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
/// Time Complexity:    O(`len_ns` * `len_s`)
/// Space Complexity:   O(`len_ns` * `len_s`)
/// Reference:
/// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let len_ns: usize = nums.len();
        let len_t: usize = target.len();
        let num_to_freq: HashMap<String, i32> = {
            let mut map: HashMap<String, i32> = HashMap::with_capacity(len_ns);
            for num in nums {
                let len_s: usize = num.len();
                if len_s < len_t {
                    *map.entry(num).or_default() += 1;
                }
            }
            map
        };
        let mut ans: i32 = 0;
        for (num, &freq) in num_to_freq.iter() {
            let len_s: usize = num.len();
            if target.starts_with(num) {
                ans += if format!("{}{}", num, num) == target {
                    freq * (freq - 1)
                } else {
                    let freq_sub = *num_to_freq.get(&target[len_s..]).unwrap_or(&0);
                    freq * freq_sub
                };
            }
        }
        return ans;
    }
}
