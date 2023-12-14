/// @author: Leon
/// https://leetcode.com/problems/difference-between-ones-and-zeros-in-row-and-column/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let (freq_r_ones, freq_c_ones): (Vec<i32>, Vec<i32>) = {
            let mut freq_r: Vec<i32> = vec![0; len_rs];
            let mut freq_c: Vec<i32> = vec![0; len_cs];
            for r in 0..len_rs {
                for c in 0..len_cs {
                    if grid[r][c] == 1 {
                        freq_r[r] += 1;
                        freq_c[c] += 1;
                    }
                }
            }
            (freq_r, freq_c)
        };
        let mut ans: Vec<Vec<i32>> = vec![vec![0; len_cs]; len_rs];
        for r in 0..len_rs {
            for c in 0..len_cs {
                let ones_r: i32 = freq_r_ones[r];
                let zeros_r: i32 = len_rs as i32 - ones_r;
                let ones_c: i32 = freq_c_ones[c];
                let zeros_c: i32 = len_cs as i32 - ones_c;
                ans[r][c] = ones_r + ones_c - zeros_r - zeros_c;
            }
        }
        return ans;
    }
}
