/// @author: Leon
/// https://leetcode.com/problems/final-value-of-variable-after-performing-operations/
/// Time Complexity:    O(`_len_os`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        const MINUS: char = '-';
        const PLUS: char = '+';
        let _len_os: usize = operations.len();
        let mut cnt_minus: i32 = 0;
        let mut cnt_plus: i32 = 0;
        for op in operations {
            let chs: Vec<char> = op.chars().collect();
            match chs[1] {
                MINUS => cnt_minus += 1,
                PLUS => cnt_plus += 1,
                _ => {}
            }
        }
        return cnt_plus - cnt_minus;
    }
}
