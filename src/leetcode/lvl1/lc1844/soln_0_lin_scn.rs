/// @author: Leon
/// https://leetcode.com/problems/replace-all-digits-with-characters/
///
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`) / O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let len_s = s.len();
        let mut chs: Vec<char> = s.chars().collect();

        for idx in 0..len_s {
            if idx % 2 == 1 {
                let prev: u8 = chs[idx - 1] as u8 - 'a' as u8;
                let shift: u8 = chs[idx] as u8 - '0' as u8;
                let new_ch: char = (prev + shift + 'a' as u8) as char;
                chs[idx] = new_ch;
            }
        }

        chs.iter().collect::<String>()
    }
}
