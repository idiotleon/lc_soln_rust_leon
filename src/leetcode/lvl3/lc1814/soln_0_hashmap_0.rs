use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/count-nice-pairs-in-an-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/count-nice-pairs-in-an-array/discuss/1140639/JavaC%2B%2BPython-Straight-Forward
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        const MOD: u64 = 1e9 as u64 + 7;
        let len_ns: usize = nums.len();
        let diff_to_freq: HashMap<i32, u32> = {
            let mut map: HashMap<i32, u32> = HashMap::with_capacity(len_ns);
            for num in nums {
                let rev = Self::reverse(num);
                let diff = num - rev;
                *map.entry(diff).or_default() += 1;
            }
            map
        };
        let mut cnt: u32 = 0;
        for (_, freq) in diff_to_freq.into_iter() {
            if freq == 1 {
                continue;
            }
            cnt += (freq as u64 * (freq as u64 - 1) / 2 % MOD) as u32;
            cnt %= MOD as u32;
        }
        return cnt as i32;
    }
    fn reverse(num: i32) -> i32 {
        let mut num: i32 = num;
        let mut ans: i32 = 0;
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            ans *= 10;
            ans += digit;
        }
        return ans;
    }
}
