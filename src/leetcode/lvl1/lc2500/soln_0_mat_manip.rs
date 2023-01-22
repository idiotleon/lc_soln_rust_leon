/// @author: Leon
/// https://leetcode.com/problems/delete-greatest-value-in-each-row/
/// Time Complexity:    O(`_len_rs` * lg(`_len_rs`) * lg(`len_cs`)) + O(`_len_rs` * `len_cs`) ~ O(`_len_rs` * lg(`_len_rs`) * lg(`len_cs`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let _len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let sorted: Vec<Vec<i32>> = {
            let mut sorted = grid;
            sorted.iter_mut().for_each(|r| r.sort());
            sorted
        };
        let mut sum: i32 = 0;
        for c in 0..len_cs {
            sum += Self::get_max_by_col(c, &sorted);
        }
        return sum;
    }
    fn get_max_by_col(c: usize, grid: &Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let mut max = grid[0][c];
        for r in 1..len_rs {
            max = std::cmp::max(max, grid[r][c]);
        }
        return max;
    }
}
