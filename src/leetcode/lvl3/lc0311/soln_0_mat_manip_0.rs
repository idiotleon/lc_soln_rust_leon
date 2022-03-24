/// @author: Leon
/// https://leetcode.com/problems/sparse-matrix-multiplication/
/// Time Complexity:    O(`len_r1` * `len_c2` * `len_c1`)
/// Space Complexity:   O(`len_r1` * `len_c2`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_r1: usize = mat1.len();
        // `len_c1` should always be equal to `_len_c2`
        let len_c1: usize = mat1[0].len();
        let _len_r2: usize = mat2.len();
        let len_c2: usize = mat2[0].len();
        let mut ans: Vec<Vec<i32>> = vec![vec![0; len_c2]; len_r1];
        for r1 in 0..len_r1 {
            for c2 in 0..len_c2 {
                let mut sum: i32 = 0;
                for idx in 0..len_c1 {
                    sum += mat1[r1][idx] * mat2[idx][c2];
                }
                ans[r1][c2] = sum;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mat1: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![-1, 0, 3]];
        let mat2: Vec<Vec<i32>> = vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        let actual: Vec<Vec<i32>> = Solution::multiply(mat1, mat2);
        let expected: Vec<Vec<i32>> = vec![vec![7, 0, 0], vec![-7, 0, 3]];
        assert_eq!(expected, actual);
    }
}
