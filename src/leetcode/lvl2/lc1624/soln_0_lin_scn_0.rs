/// @author: Leon
/// https://leetcode.com/problems/largest-substring-between-two-equal-characters/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let _len_s: usize = s.len();
        let mut largest: i32 = -1;
        let mut ch_to_idx = {
            let mut res: Vec<isize> = vec![-1; 26];
            for (idx, ch) in s.chars().into_iter().enumerate(){
                let idx_ch: usize = ch as usize - 'a' as usize;
                if res[idx_ch] > 0{
                    largest = std::cmp::max(largest, (idx - res[idx_ch] as usize) as i32);
                }else{
                    res[idx_ch] = idx as isize + 1;
                }
            }
            res
        };
        largest
    }
}
