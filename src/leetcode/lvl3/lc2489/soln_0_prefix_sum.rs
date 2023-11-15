use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/number-of-substrings-with-fixed-ratio/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/number-of-substrings-with-fixed-ratio/solutions/2861416/c-java-python3-prefix-sum-freq-table/
/// https://leetcode.com/problems/number-of-substrings-with-fixed-ratio/solutions/2861416/c-java-python3-prefix-sum-freq-table/comments/1722716
/// https://leetcode.com/problems/number-of-substrings-with-fixed-ratio/solutions/2861416/c-java-python3-prefix-sum-freq-table/comments/2037790
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fixed_ratio(s: String, num1: i32, num2: i32) -> i64 {
        let len_s: usize = s.len();
        let mut prefix_to_freq: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(len_s);
            map.insert(0, 1);
            map
        };
        let mut ans: i64 = 0;
        let mut prefix: i32 = 0;
        for ch in s.chars() {
            match ch {
                '0' => prefix += num2,
                '1' => prefix -= num1,
                _ => {}
            }
            let freq = prefix_to_freq.entry(prefix).or_default();
            ans += *freq as i64;
            *freq += 1;
        }
        return ans;
    }
}
