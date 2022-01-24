/// @author: Leon
/// https://leetcode.com/problems/largest-substring-between-two-equal-characters/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let _len_s: usize = s.len();
        let ch_to_indices: Vec<Vec<u16>> = {
            let mut res: Vec<Vec<u16>> = vec![Vec::with_capacity(2); 26];
            for (idx, ch) in s.chars().into_iter().enumerate() {
                let idx_ch: usize = ch as usize - 'a' as usize;
                let indices = &mut res[idx_ch];
                if indices.len() == 2 {
                    indices.pop();
                }
                indices.push(idx as u16);
            }
            res
        };
        let mut largest: i32 = -1;
        for indices in ch_to_indices {
            let len: usize = indices.len();
            if len == 2 {
                largest = std::cmp::max(largest, (indices[1] - indices[0]) as i32 - 1);
            }
        }
        largest
    }
}
