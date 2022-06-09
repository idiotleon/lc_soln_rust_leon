/// @author: Leon
/// https://leetcode.com/problems/find-k-length-substrings-with-no-repeated-characters/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let ku: usize = k as usize;
        if ku > len_s {
            return 0;
        }
        let mut ans: u16 = 0;
        let mut cnt_rep: u16 = 0;
        let mut freqs: Vec<u16> = {
            let mut freqs = vec![0; 26];
            for idx in 0..ku {
                let idx_ch: usize = chs[idx] as usize - 'a' as usize;
                if freqs[idx_ch] > 0 {
                    cnt_rep += 1;
                }
                freqs[idx_ch] += 1;
            }
            freqs
        };
        if cnt_rep == 0 {
            ans += 1;
        }
        for idx in ku..len_s {
            let idx_ch_hi: usize = chs[idx] as usize - 'a' as usize;
            if freqs[idx_ch_hi] > 0 {
                cnt_rep += 1;
            }
            freqs[idx_ch_hi] += 1;
            let idx_ch_lo: usize = chs[idx - ku] as usize - 'a' as usize;
            if freqs[idx_ch_lo] > 1 {
                cnt_rep -= 1;
            }
            freqs[idx_ch_lo] -= 1;
            if cnt_rep == 0 {
                ans += 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_1_should_return_expected() {
        let s: String = "havefunonleetcode".to_owned();
        let k: i32 = 5;
        let actual = Solution::num_k_len_substr_no_repeats(s, k);
        let expected: i32 = 6;
        assert_eq!(expected, actual);
    }
}
