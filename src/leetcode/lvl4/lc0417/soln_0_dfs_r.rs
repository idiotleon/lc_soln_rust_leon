/// @author: Leon
/// https://leetcode.com/problems/pacific-atlantic-water-flow/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    pub fn pacific_atlantic(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len_rs: usize = mat.len();
        let len_cs: usize = mat[0].len();
        let pac: Vec<Vec<bool>> = {
            let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
            for r in 0..len_rs as isize {
                Self::dfs((r, 0), &mut visited, 0, &mat);
            }
            for c in 0..len_cs as isize {
                Self::dfs((0, c), &mut visited, 0, &mat);
            }
            visited
        };
        let atl: Vec<Vec<bool>> = {
            let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
            for r in 0..len_rs as isize {
                Self::dfs((r, len_cs as isize - 1), &mut visited, 0, &mat);
            }
            for c in 0..len_cs as isize {
                Self::dfs((len_rs as isize - 1, c), &mut visited, 0, &mat);
            }
            visited
        };
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for r in 0..len_rs {
            for c in 0..len_cs {
                if pac[r][c] && atl[r][c] {
                    ans.push(vec![r as i32, c as i32]);
                }
            }
        }
        return ans;
    }
    fn dfs(coord: (isize, isize), visited: &mut Vec<Vec<bool>>, prev: i32, mat: &Vec<Vec<i32>>) {
        let len_rs: isize = mat.len() as isize;
        let len_cs: isize = mat[0].len() as isize;
        let (r, c) = coord;
        if r < 0
            || r >= len_rs
            || c < 0
            || c >= len_cs
            || visited[r as usize][c as usize]
            || mat[r as usize][c as usize] < prev
        {
            return;
        }
        visited[r as usize][c as usize] = true;
        for d in 0..4 {
            let r_nxt: isize = r + Self::DIRS[d];
            let c_nxt: isize = c + Self::DIRS[d + 1];
            Self::dfs((r_nxt, c_nxt), visited, mat[r as usize][c as usize], mat);
        }
    }
}
