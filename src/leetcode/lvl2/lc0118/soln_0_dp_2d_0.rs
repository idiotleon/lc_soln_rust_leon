/// @author: Leon
/// https://leetcode.com/problems/pascals-triangle/
/// Time Complexity:    O(`num_rows` ^ 2)
/// Space Complexity:   O(1) / O(`num_rows` ^ 2)
/// Reference:
/// https://leetcode.com/problems/pascals-triangle/discuss/38171/Maybe-shortest-c%2B%2B-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let len_rs: usize = num_rows as usize;
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(len_rs);
        for r in 0..len_rs {
            let mut res: Vec<i32> = vec![1; r + 1];
            for c in 1..r {
                res[c] = ans[r - 1][c - 1] + ans[r - 1][c];
            }
            ans.push(res);
        }
        ans
    }
}
