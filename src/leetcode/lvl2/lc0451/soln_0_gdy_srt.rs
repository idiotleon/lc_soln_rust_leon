/// @author: Leon
/// https://leetcode.com/problems/sort-characters-by-frequency/
/// Time Complexity:    O(`_len` * lg(`_len`))
/// Space Complexity:   O(`_len`)
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let _len = s.len();
        let mut chs: Vec<char> = s.chars().collect();
        let freqs: Vec<u32> = {
            let mut tmp = vec![0 as u32; 256];
            for ch in s.chars() {
                tmp[ch as usize] += 1;
            }
            tmp
        };
        chs.sort_by(|&a, &b| {
            if freqs[b as usize] != freqs[a as usize] {
                freqs[b as usize].cmp(&freqs[a as usize])
            } else {
                a.cmp(&b)
            }
        });
        chs.into_iter().collect()
    }
}
