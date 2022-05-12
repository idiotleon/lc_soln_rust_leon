/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const VOWELS: &'static [char] = &['a', 'e', 'i', 'o', 'u'];
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let _len_s: usize = s.len();
        let ku: usize = k as usize;
        let chs: Vec<char> = s.chars().collect();
        let mut cnt: i32 = 0;
        let mut most: i32 = 0;
        for (idx, &ch) in chs.iter().enumerate() {
            if Self::is_vowel(ch) {
                cnt += 1;
            }
            most = std::cmp::max(most, cnt);
            if idx + 1 >= ku {
                if Self::is_vowel(chs[idx + 1 - ku]) {
                    cnt -= 1;
                }
            }
        }
        most
    }
    fn is_vowel(ch: char) -> bool {
        for &vw in Self::VOWELS {
            if vw == ch {
                return true;
            }
        }
        false
    }
}
