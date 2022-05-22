/// @author: Leon
/// https://leetcode.com/problems/find-all-anagrams-in-a-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`) + O(26) ~ O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let len_s: usize = s.len();
        let len_p: usize = p.len();
        let mut ans: Vec<i32> = Vec::new();
        // sanity check
        if len_p > len_s {
            return ans;
        }
        let chs_s: Vec<char> = s.chars().collect();
        let mut cnt: u16 = len_p as u16;
        let mut freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; 26];
            for ch in p.chars() {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let mut lo: usize = 0;
        for (hi, &ch) in chs_s.iter().enumerate() {
            let freq = &mut freqs[ch as usize - 'a' as usize];
            if *freq > 0 {
                cnt -= 1;
            }
            if cnt == 0 {
                ans.push(lo as i32);
            }
            *freq -= 1;
            if hi - lo + 1 == len_p {
                let freq = &mut freqs[chs_s[lo] as usize - 'a' as usize];
                if *freq >= 0 {
                    cnt += 1;
                }
                *freq += 1;
                lo += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let s = "cbaebabacd".to_owned();
        let p = "abc".to_owned();
        let actual = Solution::find_anagrams(s, p);
        let expected: Vec<i32> = vec![0, 6];
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let s = "abab".to_owned();
        let p = "ab".to_owned();
        let actual = Solution::find_anagrams(s, p);
        let expected: Vec<i32> = vec![0, 1, 2];
        assert_eq!(expected, actual);
    }
}
