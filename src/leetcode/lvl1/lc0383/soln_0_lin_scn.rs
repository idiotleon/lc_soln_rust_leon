/// @author: Leon
/// https://leetcode.com/problems/ransom-note/
/// Time Complexity:    O(max(`_len_rn`, `_len_m`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let _len_rn: usize = ransom_note.len();
        let _len_m: usize = magazine.len();
        let mut freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; 26];
            for ch in magazine.chars() {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        for ch in ransom_note.chars() {
            let freq = &mut freqs[ch as usize - 'a' as usize];
            *freq -= 1;
            if *freq < 0 {
                return false;
            }
        }
        return true;
    }
}
