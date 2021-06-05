/// @author: Leon
/// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
/// 
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// 
/// Reference:
/// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/discuss/419402/JavaC%2B%2B-Stack
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        // not used
        // let len_s = s.len();

        const PLACE_HOLDER: char = '#';
        const PAREN_OPEN: char = '(';
        const PAREN_CLOSED: char = ')';
        
        let mut chs: Vec<char> = s.chars().collect();
        
        let mut stack = VecDeque::<usize>::new();
        
        for (idx, ch) in s.chars().into_iter().enumerate(){
            match ch{
                PAREN_OPEN => stack.push_back(idx),
                PAREN_CLOSED => {
                    if stack.is_empty(){
                        chs[idx] = PLACE_HOLDER;
                    }else{
                        stack.pop_back();
                    }
                },
                _ => (),
            };
        }
        
        while let Some(idx) = stack.pop_back(){
            chs[idx] = PLACE_HOLDER;
        }
        
        chs.into_iter().filter(|&c| c != PLACE_HOLDER).collect::<String>()
    }
}