/// @author: Leon
/// https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/
/// 
/// Time Complexity:    O(`len_s`)
/// Space Compleixty:   O(26) ~ O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        // not used
        // let len_s = s.len();

        let freqs: Vec<u16> = {
            let mut tmp: Vec<u16> = vec![0; 26];
            for ch in s.chars(){
                let idx = ch as u8 - 'a' as u8;
                tmp[idx as usize] += 1;
            }
            tmp
        };
        
        let mut prev_cnt = 0;
        for freq in freqs{
            if freq > 0{
                if prev_cnt > 0 {
                    if freq != prev_cnt{
                        return false;
                    }
                }
                
                prev_cnt = freq;
            }
        }
        
        true
    }
}