/// @author: Leon
/// https://leetcode.com/problems/check-if-the-sentence-is-pangram/
/// 
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        // not used
        // let len_s = sentence.len();

        let mut bit_mask: u32 = 0;
        let mut cnt = 0;
        
        for ch in sentence.chars(){
            let idx = ch as usize - 'a' as usize;
            if bit_mask & (1 << idx) == 0{
                cnt += 1;
            }
            bit_mask = bit_mask | (1 << idx);
        }
        
        cnt == 26
    }
}