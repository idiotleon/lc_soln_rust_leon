/// https://leetcode.com/problems/find-a-peak-element-ii/
/// Time Complexity:    O(`_len_r` * lg(`len_c`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/find-a-peak-element-ii/discuss/1276556/JavaPythonC%2B%2B-Clear-Explanation-with-Images-or-M*Log(N)-or-Very-Short-code
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let _len_r: usize = mat.len();
        let len_c: usize = mat[0].len();
        let mut lo_c: usize = 0;
        let mut hi_c: usize = len_c - 1;
        while lo_c <= hi_c {
            let mid_c = lo_c + (hi_c - lo_c) / 2;
            let max_r: usize = {
                let mut max_r: usize = 0;
                for (idx_r, nums) in mat.iter().enumerate() {
                    max_r = if nums[mid_c] >= mat[max_r][mid_c] {
                        idx_r
                    } else {
                        max_r
                    }
                }
                max_r
            };
            let is_left_larger: bool =
                mid_c as isize - 1 >= lo_c as isize && mat[max_r][mid_c - 1] > mat[max_r][mid_c];
            let is_right_larger: bool =
                mid_c + 1 <= hi_c && mat[max_r][mid_c + 1] > mat[max_r][mid_c];
            if !is_left_larger && !is_right_larger {
                return vec![max_r as i32, mid_c as i32];
            } else if is_right_larger {
                lo_c = mid_c + 1;
            } else {
                hi_c = mid_c - 1;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mat: Vec<Vec<i32>> = vec![vec![1, 4], vec![3, 2]];
        let actual = Solution::find_peak_grid(mat);
        let all: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let expected: bool = true;
        assert_eq!(expected, all.contains(&actual));
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let mat: Vec<Vec<i32>> = vec![vec![10, 20, 15], vec![21, 30, 14], vec![7, 16, 32]];
        let actual = Solution::find_peak_grid(mat);
        let all: Vec<Vec<i32>> = vec![vec![1, 1], vec![2, 2]];
        let expected: bool = true;
        assert_eq!(expected, all.contains(&actual));
    }
}
