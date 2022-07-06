/// @author: Leon
/// https://leetcode.com/problems/largest-3-same-digit-number-in-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_good_integer(num: String, k: i32) -> String {
        let len_s: usize = num.len();
        let k: usize = k as usize;
        const IMPL: char = '#';
        let chs: Vec<char> = num.chars().collect();
        let mut largest: char = IMPL;
        let mut freqs: Vec<u16> = vec![0; 26];
        let mut distinct: u8 = 0;
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        while hi < len_s {
            let idx_ch_hi: usize = chs[hi] as usize - '0' as usize;
            if freqs[idx_ch_hi] == 0 {
                distinct += 1;
            }
            freqs[idx_ch_hi] += 1;
            if hi + 1 >= k {
                if distinct == 1 {
                    if largest == IMPL || chs[hi] > largest {
                        largest = chs[hi]
                    }
                }
                let idx_ch_lo: usize = chs[lo] as usize - '0' as usize;
                freqs[idx_ch_lo] -= 1;
                if freqs[idx_ch_lo] == 0 {
                    distinct -= 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        if largest != IMPL {
            largest.to_string().repeat(k)
        } else {
            "".to_owned()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let num: String = "6777133339".to_owned();
        let expected: String = "777".to_owned();
        let actual = Solution::largest_good_integer(num, 3);
        assert_eq!(expected, actual);
    }
}
