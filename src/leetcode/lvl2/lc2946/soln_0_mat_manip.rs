/// @author: Leon
/// https://leetcode.com/problems/matrix-similarity-after-cyclic-shifts/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let len_rs: usize = mat.len();
        let len_cs: usize = mat[0].len();
        for r in 0..len_rs {
            for c in 0..len_cs {
                if r % 2 == 0 {
                    let c_nxt: usize = {
                        let mut res: isize = c as isize;
                        res -= k as isize;
                        res %= len_cs as isize;
                        if res < 0 {
                            res += len_cs as isize;
                        }
                        res as usize
                    };
                    if mat[r][c] != mat[r][c_nxt] {
                        return false;
                    }
                } else {
                    let c_nxt: usize = (c + k as usize) % len_cs;
                    if mat[r][c] != mat[r][c_nxt] {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mat: Vec<Vec<i32>> = vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]];
        let k: i32 = 2;
        let expected: bool = true;
        let actual: bool = Solution::are_similar(mat, k);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let mat: Vec<Vec<i32>> = vec![vec![2, 2], vec![2, 2]];
        let k: i32 = 3;
        let expected: bool = true;
        let actual: bool = Solution::are_similar(mat, k);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let mat: Vec<Vec<i32>> = vec![vec![1, 2]];
        let k: i32 = 1;
        let expected: bool = false;
        let actual: bool = Solution::are_similar(mat, k);
        assert_eq!(expected, actual);
    }
}
