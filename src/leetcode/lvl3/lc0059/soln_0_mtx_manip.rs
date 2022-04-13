/// @author: Leon
/// https://leetcode.com/problems/spiral-matrix-ii/
/// Time Complexity:    O(`n` ^ 2)
/// Space Complexity:   O(`n` ^ 2) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n: usize = n as usize;
        if n == 1 {
            return vec![vec![1]];
        }
        let mut cur: i32 = 1;
        let mut ans = vec![vec![0; n]; n];
        let mut lo: usize = 0;
        let mut hi: usize = n - 1;
        let mut top: usize = 0;
        let mut btm: usize = n - 1;
        while lo <= hi {
            for c in lo..=hi {
                ans[top][c] = cur;
                cur += 1;
            }
            top += 1;
            for r in top..=btm {
                ans[r][hi] = cur;
                cur += 1;
            }
            hi -= 1;
            for c in (lo..=hi).rev() {
                ans[btm][c] = cur;
                cur += 1;
            }
            btm -= 1;
            for r in (top..=btm).rev() {
                ans[r][lo] = cur;
                cur += 1;
            }
            lo += 1;
        }
        ans
    }
}
