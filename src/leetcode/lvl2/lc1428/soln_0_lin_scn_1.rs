use crate::leetcode::lvl2::lc1428::fake_binary_mat::FakeBinaryMatrix;

/// @author: Leon
/// https://leetcode.com/problems/leftmost-column-with-at-least-a-one/description/
/// Time Complexity:    O(`len_rs` + `len_cs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn left_most_column_with_one(mat: &FakeBinaryMatrix) -> i32 {
        let dimens: Vec<i32> = mat.dimensions();
        let len_rs: i32 = dimens[0];
        let len_cs: i32 = dimens[1];
        let mut r: i32 = 0;
        let mut c: i32 = len_cs - 1;
        while r < len_rs && c >= 0 {
            if mat.get(r, c) == 1 {
                c -= 1;
            } else {
                r += 1;
            }
        }
        return if c == len_cs - 1 { -1 } else { c + 1 };
    }
}
