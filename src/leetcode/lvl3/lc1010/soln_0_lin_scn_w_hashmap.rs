use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/discuss/256726/JavaPython-3-O(n)-code-w-comment-similar-to-Two-Sum/644125
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/discuss/256726/JavaPython-3-O(n)-code-w-comment-similar-to-Two-Sum
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_pairs_divisible_by60(times: Vec<i32>) -> i32 {
        let _len_t: usize = times.len();
        const MOD: i32 = 60;
        let mut modded_time_to_freq: HashMap<i32, i32> = HashMap::new();
        let mut cnt: i32 = 0;
        for time in times {
            let modded = time % MOD;
            let expected = if modded == 0 { 0 } else { MOD - modded };
            cnt += modded_time_to_freq.get(&expected).unwrap_or(&0);
            *modded_time_to_freq.entry(modded).or_default() += 1;
        }
        cnt
    }
}
