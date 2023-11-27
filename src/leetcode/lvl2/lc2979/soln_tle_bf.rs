/// @author: Leon
/// https://leetcode.com/problems/count-beautiful-substrings-i/
/// Time Complexity:    O(`len_s` ^ 3)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut cnt: i32 = 0;
        for lo in 0..len_s - 1 {
            for hi in lo + 1..len_s {
                let (cnt_vowel, cnt_con) = Self::count(lo, hi, &chs);
                if cnt_vowel == cnt_con && (cnt_vowel * cnt_con) % k == 0 {
                    cnt += 1;
                }
            }
        }
        return cnt;
    }
    fn count(lo: usize, hi: usize, chs: &Vec<char>) -> (i32, i32) {
        let mut cnt_vowel: i32 = 0;
        for idx in lo..=hi {
            if Self::is_vowel(chs[idx]) {
                cnt_vowel += 1;
            };
        }
        return (cnt_vowel, (hi - lo + 1) as i32 - cnt_vowel);
    }
    fn is_vowel(ch: char) -> bool {
        let all: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        for c in all {
            if ch == c {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s: String = "baeyh".to_owned();
        let k: i32 = 2;
        let expected: i32 = 2;
        let actual: i32 = Solution::beautiful_substrings(s, k);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let s: String = "abba".to_owned();
        let k: i32 = 1;
        let expected: i32 = 2;
        let actual: i32 = Solution::beautiful_substrings(s, k);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let s: String = "dcdf".to_owned();
        let k: i32 = 1;
        let expected: i32 = 2;
        let actual: i32 = Solution::beautiful_substrings(s, k);
        assert_eq!(expected, actual);
    }
}
