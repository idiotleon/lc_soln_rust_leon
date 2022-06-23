/// @author: Leon
/// https://leetcode.com/problems/rotate-image/
/// Time Complexity:     O(`len_r` * `len_c`)
/// Space Complexity:    O(1)
/// Reference:
/// https://leetcode.com/problems/rotate-image/discuss/18872/A-common-method-to-rotate-the-image
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len_r: usize = matrix.len();
        matrix.reverse();
        for row in 0..len_r {
            let len_c: usize = matrix[row].len();
            for col in row + 1..len_c {
                let temp = matrix[row][col];
                matrix[row][col] = matrix[col][row];
                matrix[col][row] = temp;
            }
        }
    }
}
