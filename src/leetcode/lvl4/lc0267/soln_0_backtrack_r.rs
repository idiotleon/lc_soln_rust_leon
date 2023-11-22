/// @author: Leon
/// https://leetcode.com/problems/palindrome-permutation-ii/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    const IMPS: char = '#';
    pub fn generate_palindromes(s: String) -> Vec<String> {
        let _len_s: usize = s.len();
        let mut paths: Vec<String> = Vec::new();
        let (mut freqs, half_len, ch_odd): (Vec<u8>, usize, char) = {
            let mut freqs: Vec<u8> = vec![0; 26];
            for ch in s.chars() {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            let mut cnt_odd: u8 = 0;
            let mut half_len: u8 = 0;
            let mut ch_odd: char = Self::IMPS;
            for idx in 0..26 {
                if freqs[idx] % 2 == 1 {
                    cnt_odd += 1;
                    ch_odd = (idx as u8 + 'a' as u8) as char;
                }
                freqs[idx] /= 2;
                half_len += freqs[idx];
            }
            if cnt_odd > 1 {
                return paths;
            }
            (freqs, half_len as usize, ch_odd)
        };
        let mut path: String = "".to_owned();
        Self::backtrack(&mut path, &mut freqs, &mut paths, half_len, ch_odd);
        return paths;
    }
    fn backtrack(
        path: &mut String,
        freqs: &mut Vec<u8>,
        paths: &mut Vec<String>,
        half_len: usize,
        mid_char: char,
    ) {
        if path.len() == half_len {
            let mut res = path.to_owned();
            if mid_char != Self::IMPS {
                res.push(mid_char);
            }
            let rev = path.to_owned().chars().rev().collect::<String>();
            res.push_str(&rev);
            paths.push(res);
            return;
        }
        for idx in 0..26 {
            if freqs[idx] > 0 {
                freqs[idx] -= 1;
                path.push((idx as u8 + 'a' as u8) as char);
                Self::backtrack(path, freqs, paths, half_len, mid_char);
                path.pop();
                freqs[idx] += 1;
            }
        }
    }
}
