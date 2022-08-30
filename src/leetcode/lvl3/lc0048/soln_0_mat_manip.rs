/// @author: Leon
/// https://leetcode.com/problems/rotate-image/
/// Time Complexity:     O(`len_rs` * `len_cs`)
/// Space Complexity:    O(1)
/// Reference:
/// https://leetcode.com/problems/rotate-image/discuss/18872/A-common-method-to-rotate-the-image
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len_rs: usize = matrix.len();
        matrix.reverse();
        for r in 0..len_rs {
            let len_cs: usize = matrix[r].len();
            for c in r + 1..len_cs {
                let tmp = matrix[r][c];
                matrix[r][c] = matrix[c][r];
                matrix[c][r] = tmp;
            }
        }
    }
}
