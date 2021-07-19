/// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
/// 
/// Time Complexity:    O(`len_w` * ave_len_word)
/// Space Complexity:   O(26) ~ O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let len_w = words.len();
        let freqs = {
            let mut tmp = vec![0; 26];
            
            for word in words{
                for ch in word.chars(){
                    tmp[ch as usize - 'a' as usize] += 1;
                }
            }
            
            tmp
        };
        
        for freq in freqs{
            if freq % len_w != 0{
                return false;
            }
        }
        
        true
    }
}