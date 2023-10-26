/// @author: Leon
/// https://leetcode.com/problems/spiral-matrix-ii/
/// Time Complexity:    O(`n` ^ 2)
/// Space Complexity:   O(`n` ^ 2) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 1 {
            return vec![vec![1]];
        }
        let mut top: usize = 0;
        let mut bottom: usize = n as usize - 1;
        let mut left: usize = 0;
        let mut right: usize = n as usize - 1;
        let mut cur: i32 = 1;
        let mut ans: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
        while cur <= n * n {
            for c in left..=right {
                ans[top][c] = cur;
                cur += 1;
            }
            top += 1;
            for r in top..=bottom {
                ans[r][right] = cur;
                cur += 1;
            }
            right -= 1;
            for c in (left..=right).rev() {
                ans[bottom][c] = cur;
                cur += 1;
            }
            bottom -= 1;
            for r in (top..=bottom).rev() {
                ans[r][left] = cur;
                cur += 1;
            }
            left += 1;
        }
        return ans;
    }
}
