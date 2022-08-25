/// @author: Leon
/// https://leetcode.com/problems/convert-1d-array-into-2d-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1) / O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let len_rs: usize = m as usize;
        let len_cs: usize = n as usize;
        let len_ns: usize = original.len();
        if len_rs * len_cs != len_ns {
            return Vec::new();
        }
        let mut ans: Vec<Vec<i32>> = vec![vec![0; len_cs]; len_rs];
        for (idx, num) in original.into_iter().enumerate() {
            ans[idx / len_cs][idx % len_cs] = num;
        }
        return ans;
    }
}
