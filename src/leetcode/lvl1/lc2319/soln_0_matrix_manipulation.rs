/// @author: Leon
///
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let len_n: usize = grid.len();
        for r in 0..len_n {
            for c in 0..len_n {
                if Self::is_any_diagonal(r, c, len_n) {
                    if grid[r][c] == 0 {
                        return false;
                    }
                } else {
                    if grid[r][c] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn is_any_diagonal(r: usize, c: usize, len_n: usize) -> bool {
        (r == c) || (r + c == len_n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let grid = vec![
            vec![2, 0, 0, 1],
            vec![0, 3, 1, 0],
            vec![0, 5, 2, 0],
            vec![4, 0, 0, 2],
        ];
        let actual = Solution::check_x_matrix(grid);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
