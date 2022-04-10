use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/baseball-game/
/// Time Complexity:    O(`_len_ops`)
/// Space Complexity:   O(`_len_ops`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let _len_ops: usize = ops.len();
        const C: &str = "C";
        const D: &str = "D";
        const PLUS: &str = "+";
        let mut stk: VecDeque<i32> = VecDeque::new();
        for op in ops {
            match op.as_ref() {
                C => {
                    stk.pop_back();
                }
                D => {
                    if let Some(&top) = stk.back() {
                        stk.push_back(top * 2);
                    }
                }
                PLUS => {
                    let top: i32 = stk.pop_back().unwrap();
                    let top2: i32 = stk.pop_back().unwrap();
                    let num = top + top2;
                    stk.push_back(top2);
                    stk.push_back(top);
                    stk.push_back(num);
                }
                _ => {
                    stk.push_back(op.parse::<i32>().unwrap());
                }
            }
        }
        stk.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_1() {
        let ops: Vec<String> = vec![
            "5".to_owned(),
            "2".to_owned(),
            "C".to_owned(),
            "D".to_owned(),
            "+".to_owned(),
        ];
        let actual = Solution::cal_points(ops);
        let expected = 30;
        assert_eq!(expected, actual);
    }
}
