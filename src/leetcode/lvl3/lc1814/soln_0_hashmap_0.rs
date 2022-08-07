use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/count-nice-pairs-in-an-array/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/count-nice-pairs-in-an-array/discuss/1140639/JavaC%2B%2BPython-Straight-Forward
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let len_ns: usize = nums.len();
        let mut diff_to_freq: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
        let mut cnt: i32 = 0;
        for num in nums {
            let diff = num - Self::reverse(num);
            if let Some(freq) = diff_to_freq.get(&diff) {
                cnt = (cnt + freq) % MOD;
            }
            *diff_to_freq.entry(diff).or_default() += 1;
        }
        return cnt;
    }
    fn reverse(num: i32) -> i32 {
        let mut num = num;
        let mut res = 0;
        while num > 0 {
            res *= 10;
            res += num % 10;
            num /= 10;
        }
        return res;
    }
}
