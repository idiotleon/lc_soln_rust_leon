/// @author: Leon
/// https://leetcode.com/problems/minimum-absolute-difference/
///
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(`len_n`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let len_n: usize = arr.len();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let sorted = {
            let mut tmp = arr;
            tmp.sort();
            tmp
        };
        let mut min_abs = std::i32::MAX;
        for idx in 1..len_n {
            let cur_abs = (sorted[idx] - sorted[idx - 1]).abs();

            if cur_abs < min_abs {
                ans.clear();
                ans.push(vec![sorted[idx - 1], sorted[idx]]);
                min_abs = cur_abs;
            } else if cur_abs == min_abs {
                ans.push(vec![sorted[idx - 1], sorted[idx]]);
            }
        }
        ans
    }
}
