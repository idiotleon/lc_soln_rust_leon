/// @author: Leon
/// https://leetcode.com/problems/find-all-anagrams-in-a-string/
/// Time Complexity:    O(`len_p` * `len_s`)
/// Space Complexity:   O(max(`len_p`, `len_s`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let len_s: usize = s.len();
        let len_p: usize = p.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_s);
        if len_s < len_p {
            return ans;
        }
        let chs_s: Vec<char> = s.chars().collect();
        let chs_p: Vec<char> = p.chars().collect();
        let hash_p: Vec<u16> = Self::get_hash(0, len_p, &chs_p);
        let mut idx: usize = len_p - 1;
        while idx < len_s {
            if Self::is_anagram(idx, &chs_s, &hash_p, len_p) {
                ans.push((idx + 1 - len_p) as i32);
            };
            idx += 1;
        }
        ans
    }
    fn is_anagram(hi: usize, chs_s: &Vec<char>, hash_p: &Vec<u16>, len_p: usize) -> bool {
        let lo = hi + 1 - len_p;
        let hash_s: Vec<u16> = Self::get_hash(lo, hi + 1, &chs_s);
        for (idx, hash) in hash_s.into_iter().enumerate() {
            if hash != hash_p[idx] {
                return false;
            }
        }
        true
    }
    fn get_hash(lo: usize, hi: usize, chs: &Vec<char>) -> Vec<u16> {
        let mut freqs: Vec<u16> = vec![0; 26];
        for idx in lo..hi {
            freqs[chs[idx] as usize - 'a' as usize] += 1;
        }
        freqs
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
