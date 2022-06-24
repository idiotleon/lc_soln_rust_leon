/// @author: Leon
/// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/discuss/2168117/Java-Solution-1-Pass-O(n):-Count-0s-and-1s/1445209
/// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/discuss/2168227/JavaC%2B%2BPython-One-Pass-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let len_s: usize = s.len();
        let k: i32 = k as i32;
        let mut cnt_ones: i32 = 0;
        let mut cnt_zeros: i32 = 0;
        let mut add_one: bool = true;
        let mut sum: i32 = 0;
        for (idx, &b) in s.as_bytes().iter().enumerate().rev() {
            if b == b'0' {
                cnt_zeros += 1;
            } else if add_one {
                if let Some(res) = i32::checked_pow(2, (len_s - idx - 1) as u32) {
                    sum += res;
                    if sum <= k {
                        cnt_ones += 1;
                    } else {
                        add_one = false;
                    }
                }
            }
        }
        cnt_ones + cnt_zeros
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_with_test_case_61_should_return_expected() {
        let s: String = "001010101011010100010101101010010".to_owned();
        let k: i32 = 93951055;
        let actual = Solution::longest_subsequence(s, k);
        let expected = 31;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_with_test_case_151_should_return_expected() {
        let s: String = "1111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_owned();
        let k: i32 = 1;
        let actual = Solution::longest_subsequence(s, k);
        let expected = 500;
        assert_eq!(expected, actual);
    }
}
