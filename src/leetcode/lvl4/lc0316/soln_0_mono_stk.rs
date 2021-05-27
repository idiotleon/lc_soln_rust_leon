/// https://leetcode.com/problems/remove-duplicate-letters/
/// 
/// Time Complexity:    O(L)
/// Space Complexity:   O(L)
/// 
/// Reference:
/// https://leetcode.com/problems/remove-duplicate-letters/discuss/889778/Rust-stack-solution
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut freqs:[usize; 26] = [0; 26];
        let bts = s.as_bytes();
        for &b in bts{
            freqs[(b - b'a') as usize] +=1 ;
        }
        
        let mut stack: VecDeque<char> = VecDeque::with_capacity(26);
        let mut exists: [bool; 26] = [false; 26];
        
        for &b in bts{
            let idx = (b - b'a') as usize;
            freqs[idx] -= 1;
            if exists[idx]{
                continue;
            }
            
            while let Some(&top) = stack.back(){
                let idx_top = (top as u8 - b'a') as usize;
                
                if b < top as u8 && freqs[idx_top] > 0{
                    exists[idx_top] = false;
                    stack.pop_back();
                }else{
                    break;
                }
            }
            
            stack.push_back(b as char);
            exists[idx] = true;
        }
        
        stack.iter().collect::<String>()
    }
}