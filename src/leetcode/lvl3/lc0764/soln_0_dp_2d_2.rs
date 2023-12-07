/// @author: Leon
/// https://leetcode.com/problems/largest-plus-sign/
/// Time Complexity:    O(`n` ^ 2)
/// Space Complexity:   O(`n` ^ 2)
/// Reference:
/// https://leetcode.com/problems/largest-plus-sign/solutions/2912985/just-a-runnable-solution/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n: usize = n as usize;
        let grid: Vec<Vec<i32>> = {
            let mut grid: Vec<Vec<i32>> = vec![vec![1; n]; n];
            for mine in mines {
                let r: usize = mine[0] as usize;
                let c: usize = mine[1] as usize;
                grid[r][c] = 0;
            }
            grid
        };
        let (lo, hi, top, btm): (Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<Vec<i32>>) = {
            let mut lo: Vec<Vec<i32>> = vec![vec![0; n]; n];
            let mut hi: Vec<Vec<i32>> = vec![vec![0; n]; n];
            let mut top: Vec<Vec<i32>> = vec![vec![0; n]; n];
            let mut btm: Vec<Vec<i32>> = vec![vec![0; n]; n];
            for r in 0..n {
                for c in 0..n {
                    if grid[r][c] == 1 {
                        lo[r][c] = if c == 0 { 1 } else { 1 + lo[r][c - 1] };
                        top[r][c] = if r == 0 { 1 } else { top[r - 1][c] + 1 };
                    }
                }
            }
            for r in (0..n).rev() {
                for c in (0..n).rev() {
                    if grid[r][c] == 1 {
                        hi[r][c] = if c == n - 1 { 1 } else { 1 + hi[r][c + 1] };
                        btm[r][c] = if r == n - 1 { 1 } else { 1 + btm[r + 1][c] };
                    }
                }
            }
            (lo, hi, top, btm)
        };
        let mut longest: i32 = 0;
        for r in 0..n {
            for c in 0..n {
                let len_min = std::cmp::min(
                    lo[r][c],
                    std::cmp::min(hi[r][c], std::cmp::min(top[r][c], btm[r][c])),
                );
                longest = std::cmp::max(longest, len_min);
            }
        }
        return longest;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let n: i32 = 5;
        let mines: Vec<Vec<i32>> = vec![vec![4, 2]];
        let expected: i32 = 2;
        let actual: i32 = Solution::order_of_largest_plus_sign(n, mines);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let n: i32 = 1;
        let mines: Vec<Vec<i32>> = vec![vec![0, 0]];
        let expected: i32 = 2;
        let actual: i32 = Solution::order_of_largest_plus_sign(n, mines);
        assert_eq!(expected, actual);
    }
}
