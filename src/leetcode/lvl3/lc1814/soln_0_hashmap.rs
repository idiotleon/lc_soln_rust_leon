use std::collections::HashMap;

/// https://leetcode.com/problems/count-nice-pairs-in-an-array/
/// Time Complexity:    O(`_len_n * lg10(avg_len_num)`)
/// Space Complexity:   O(`_len_n`)
/// Reference:
/// https://leetcode.com/problems/count-nice-pairs-in-an-array/discuss/1140487/Count-Frequency-of-difference-of-number-and-its-reverse-or-Easy-Hashmap-Explained
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MOD: u64 = 1e9 as u64 + 7;
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut num_to_freq: HashMap<i32, u32> = HashMap::new();
        for num in nums {
            let rev = Self::reverse(num);
            let diff = num - rev;
            *num_to_freq.entry(diff).or_insert(0) += 1;
        }
        let mut cnt: u32 = 0;
        for (_, freq) in num_to_freq.into_iter() {
            if freq == 1 {
                continue;
            }
            cnt += (freq as u64 * (freq as u64 - 1) / 2 % Self::MOD) as u32;
            cnt %= Self::MOD as u32;
        }
        cnt as i32
    }
    fn reverse(mut num: i32) -> i32 {
        let mut ans: i32 = 0;
        while num != 0 {
            let digit = num % 10;
            num /= 10;
            ans *= 10;
            ans += digit;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1() {
        let nums: Vec<i32> = vec![42, 11, 1, 97];
        let actual: i32 = Solution::count_nice_pairs(nums);
        let expected: i32 = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_helper_reverse() {
        let num: i32 = 123;
        let actual: i32 = Solution::reverse(num);
        let expected: i32 = 321;
        assert_eq!(expected, actual);
    }
}
