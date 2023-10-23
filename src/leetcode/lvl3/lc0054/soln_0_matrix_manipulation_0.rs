/// @author: Leon
/// https://leetcode.com/problems/spiral-matrix/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(1) / O(`len_rs` * `len_cs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let len_rs: usize = matrix.len();
        let len_cs: usize = matrix[0].len();
        let mut top: isize = 0;
        let mut left: isize = 0;
        let mut bottom: isize = len_rs as isize - 1;
        let mut right: isize = len_cs as isize - 1;
        let mut ans: Vec<i32> = Vec::with_capacity(len_rs * len_cs);
        loop {
            for c in left..=right {
                ans.push(matrix[top as usize][c as usize]);
            }
            top += 1;
            if top > bottom || left > right {
                break;
            }
            for r in top..=bottom {
                ans.push(matrix[r as usize][right as usize]);
            }
            right -= 1;
            if top > bottom || left > right {
                break;
            }
            for c in (left..=right).rev() {
                ans.push(matrix[bottom as usize][c as usize]);
            }
            bottom -= 1;
            if top > bottom || left > right {
                break;
            }
            for r in (top..=bottom).rev() {
                ans.push(matrix[r as usize][left as usize]);
            }
            left += 1;
            if top > bottom || left > right {
                break;
            }
        }
        return ans;
    }
}
