/// @author: Leon
/// https://leetcode.com/problems/sort-vowels-in-a-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`) / O(1)
/// Reference:
/// https://leetcode.com/problems/sort-vowels-in-a-string/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    const SORTED_ALL_VOWELS: &'static [char] = &['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];
    pub fn sort_vowels(s: String) -> String {
        let len_s: usize = s.len();
        let mut chs: Vec<char> = s.chars().collect();
        let mut counts: Vec<u16> = {
            let mut res: Vec<u16> = vec![0; 256];
            for &ch in &chs {
                if Self::is_vowel(ch) {
                    res[ch as usize - 'A' as usize] += 1;
                }
            }
            res
        };
        let mut idx_c: usize = 0;
        for idx in 0..len_s {
            if Self::is_vowel(chs[idx]) {
                while counts[Self::SORTED_ALL_VOWELS[idx_c] as usize - 'A' as usize] == 0 {
                    idx_c += 1;
                }
                chs[idx] = Self::SORTED_ALL_VOWELS[idx_c];
                counts[Self::SORTED_ALL_VOWELS[idx_c] as usize - 'A' as usize] -= 1;
            } else {
                // to explicitly do nothing
            }
        }
        return chs.into_iter().collect();
    }
    fn is_vowel(ch: char) -> bool {
        for &c in Self::SORTED_ALL_VOWELS {
            if c == ch {
                return true;
            }
        }
        return false;
    }
}
