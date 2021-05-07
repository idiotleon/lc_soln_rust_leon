/// https://leetcode.com/problems/rotate-image/
///
/// Time Complexity:     O(`rows` * `cols`)
/// Space Complexity:    O(1)
///
/// Reference:
///  https://leetcode.com/problems/rotate-image/discuss/18872/A-common-method-to-rotate-the-image
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let rows: usize = matrix.len();

        matrix.reverse();

        for row in 0..rows {
            let cols: usize = matrix[row].len();
            for col in row + 1..cols {
                let temp = matrix[row][col];
                matrix[row][col] = matrix[col][row];
                matrix[col][row] = temp;
            }
        }
    }
}
