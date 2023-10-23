/// @author: Leon
/// https://leetcode.com/problems/spiral-matrix/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(1) / O(`len_rs` * `len_cs` )
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let len_rs: usize = matrix.len();
        let len_cs: usize = matrix[0].len();
        let mut r_start: isize = 0;
        let mut r_end: isize = len_rs as isize - 1;
        let mut c_start: isize = 0;
        let mut c_end: isize = len_cs as isize - 1;
        let mut ans: Vec<i32> = Vec::with_capacity(len_rs * len_cs);
        while r_start <= r_end && c_start <= c_end {
            for c in c_start..=c_end {
                ans.push(matrix[r_start as usize][c as usize]);
            }
            r_start += 1;
            for r in r_start..=r_end {
                ans.push(matrix[r as usize][c_end as usize]);
            }
            c_end -= 1;
            if r_start <= r_end {
                for c in (c_start..=c_end).rev() {
                    ans.push(matrix[r_end as usize][c as usize]);
                }
            }
            r_end -= 1;
            if c_start <= c_end {
                for r in (r_start..=r_end).rev() {
                    ans.push(matrix[r as usize][c_start as usize]);
                }
            }
            c_start += 1;
        }
        return ans;
    }
}
