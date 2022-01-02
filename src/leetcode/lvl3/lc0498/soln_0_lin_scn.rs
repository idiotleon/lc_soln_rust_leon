/// https://leetcode.com/problems/diagonal-traverse/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(1) / O(`len_r` * `len_c`)
/// Reference:
/// https://leetcode.com/problems/diagonal-traverse/discuss/97711/Java-15-lines-without-using-boolean
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let len_r: usize = mat.len();
        let len_c: usize = mat[0].len();
        let ans: Vec<i32> = {
            let len: usize = len_r * len_c;
            let mut res: Vec<i32> = vec![0; len];
            let mut idx_r: usize = 0;
            let mut idx_c: usize = 0;
            let mut idx: usize = 0;
            while idx < len {
                res[idx] = mat[idx_r][idx_c];
                idx += 1;
                if (idx_r + idx_c) % 2 == 0 {
                    if idx_c == len_c - 1 {
                        idx_r += 1;
                    } else if idx_r == 0 {
                        idx_c += 1;
                    } else {
                        idx_r -= 1;
                        idx_c += 1;
                    }
                } else {
                    if idx_r == len_r - 1 {
                        idx_c += 1;
                    } else if idx_c == 0 {
                        idx_r += 1;
                    } else {
                        idx_r += 1;
                        idx_c -= 1;
                    }
                }
            }
            res
        };
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mat: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let actual = Solution::find_diagonal_order(mat);
        let expected = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];
        assert_eq!(expected, actual);
    }
}
