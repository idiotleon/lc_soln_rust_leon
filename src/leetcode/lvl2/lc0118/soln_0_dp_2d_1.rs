/// @author: Leon
/// https://leetcode.com/problems/pascals-triangle/
/// Time Complexity:    O(`num_rows` ^ 2)
/// Space Complexity:   O(1) / O(`num_rows` ^ 2)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let len_rs: usize = num_rows as usize;
        if len_rs == 1 {
            return vec![vec![1]];
        }
        let mut ans: Vec<Vec<i32>> = {
            let mut vec: Vec<Vec<i32>> = Vec::with_capacity(len_rs);
            vec.push(vec![1]);
            vec
        };
        for r in 1..len_rs {
            let mut res: Vec<i32> = vec![1; r + 1];
            let len_prev: usize = ans[r - 1].len();
            for c in 1..len_prev {
                res[c] = ans[r - 1][c - 1] + ans[r - 1][c];
            }
            ans.push(res);
        }
        ans
    }
}
