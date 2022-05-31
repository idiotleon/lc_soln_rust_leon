use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/number-of-distinct-islands/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * `len_c`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const WATER: i32 = 0;

    const DIRS: &'static [isize] = &[0, 1, 0, -1, 0];
    const DIRS_CH: &'static [char] = &['r', 'd', 'l', 'u'];

    const START: char = 's';
    const END: char = 'e';

    pub fn num_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();

        let mut seen: HashSet<String> = HashSet::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_c]; len_r];

        for r in 0..len_r {
            for c in 0..len_c {
                if grid[r][c] == Self::WATER || visited[r][c] {
                    continue;
                }
                let mut hash: String = "".to_owned();
                Self::dfs(
                    r as isize,
                    c as isize,
                    Self::START,
                    &mut hash,
                    &mut visited,
                    &grid,
                );
                seen.insert(hash);
            }
        }

        seen.len() as i32
    }

    fn dfs(
        r: isize,
        c: isize,
        dir_ch: char,
        hash: &mut String,
        visited: &mut Vec<Vec<bool>>,
        grid: &Vec<Vec<i32>>,
    ) {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        if r < 0
            || c < 0
            || r as usize >= len_r
            || c as usize >= len_c
            || grid[r as usize][c as usize] == Self::WATER
            || visited[r as usize][c as usize]
        {
            return;
        }
        visited[r as usize][c as usize] = true;
        hash.push(dir_ch);

        for d in 0..4 {
            let nxt_r: isize = r + Self::DIRS[d];
            let nxt_c: isize = c + Self::DIRS[d + 1];

            Self::dfs(nxt_r, nxt_c, Self::DIRS_CH[d], hash, visited, grid);
        }

        hash.push(Self::END);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_1_should_return_expected() {
        let grid = vec![
            vec![1, 1, 0, 0, 0],
            vec![1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 1, 1],
        ];
        let expected = 1;
        let actual = Solution::num_distinct_islands(grid);
        assert_eq!(expected, actual);
    }
}
