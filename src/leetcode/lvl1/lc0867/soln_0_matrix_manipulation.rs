/// @author: Leon
/// https://leetcode.com/problems/transpose-matrix/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_rs: usize = matrix.len();
        let len_cs: usize = matrix[0].len();
        let mut ans: Vec<Vec<i32>> = vec![vec![0; len_rs]; len_cs];
        for r in 0..len_rs {
            for c in 0..len_cs {
                ans[c][r] = matrix[r][c];
            }
        }
        ans
    }
}
