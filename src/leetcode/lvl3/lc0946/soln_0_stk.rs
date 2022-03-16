use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/validate-stack-sequences/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let _len_n: usize = pushed.len();
        let mut stk: VecDeque<i32> = VecDeque::new();
        let mut idx: usize = 0;
        for num in pushed{
            stk.push_back(num);
            while let Some(&top) = stk.back(){
                if top == popped[idx]{
                    stk.pop_back();
                    idx += 1;
                }else{
                    break;
                }
            }
        }
        stk.is_empty()
    }
}