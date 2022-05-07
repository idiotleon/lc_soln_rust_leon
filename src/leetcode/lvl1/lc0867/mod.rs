/// @author: Leon
/// https://leetcode.com/problems/transpose-matrix/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * `len_c`) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_r: usize = matrix.len();
        let len_c: usize = matrix[0].len();
        let mut ans: Vec<Vec<i32>> = vec![vec![0; len_r]; len_c];
        for r in 0..len_r {
            for c in 0..len_c {
                ans[c][r] = matrix[r][c];
            }
        }
        ans
    }
}
