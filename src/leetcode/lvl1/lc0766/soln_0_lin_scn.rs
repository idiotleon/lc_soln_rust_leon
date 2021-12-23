/// https://leetcode.com/problems/toeplitz-matrix/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Copmlexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/toeplitz-matrix/discuss/113417/Java-solution-4-liner.
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let len_r = matrix.len();
        let len_c = matrix[0].len();
        for r in 0..len_r - 1 {
            for c in 0..len_c - 1 {
                if matrix[r][c] != matrix[r + 1][c + 1] {
                    return false;
                }
            }
        }
        true
    }
}
