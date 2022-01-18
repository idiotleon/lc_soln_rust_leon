use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/shortest-bridge/
/// Time Complexity:    O(`len_r` * `len_c` )
/// Space Compleity:    O(`len_r` * `len_c`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const ISLAND: i32 = 1;
    const WATER: i32 = 0;
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_c]; len_r];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut found: bool = false;
        'outer: for r in 0..len_r {
            for c in 0..len_c {
                if grid[r][c] == Self::ISLAND {
                    Self::dfs((r, c), &mut visited, &grid, &mut queue);
                    found = true;
                }
                if found {
                    break 'outer;
                }
            }
        }
        let mut steps: u8 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r_cur, c_cur)) = queue.pop_front() {
                    for d in 0..4 {
                        let r_nxt: isize = r_cur as isize + Self::DIRS[d];
                        let c_nxt: isize = c_cur as isize + Self::DIRS[d + 1];
                        if r_nxt < 0
                            || c_nxt < 0
                            || r_nxt as usize >= len_r
                            || c_nxt as usize >= len_c
                            || visited[r_nxt as usize][c_nxt as usize]
                        {
                            continue;
                        }
                        if grid[r_nxt as usize][c_nxt as usize] == Self::ISLAND {
                            return steps as i32;
                        }
                        queue.push_back((r_nxt as usize, c_nxt as usize));
                        visited[r_nxt as usize][c_nxt as usize] = true;
                    }
                }
            }
            steps += 1;
        }
        steps as i32
    }
    fn dfs(
        coord: (usize, usize),
        visited: &mut Vec<Vec<bool>>,
        grid: &Vec<Vec<i32>>,
        queue: &mut VecDeque<(usize, usize)>,
    ) {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let (r_cur, c_cur) = coord;
        queue.push_back((r_cur as usize, c_cur as usize));
        visited[r_cur][c_cur] = true;
        for d in 0..4 {
            let r_nxt: isize = r_cur as isize + Self::DIRS[d];
            let c_nxt: isize = c_cur as isize + Self::DIRS[d + 1];
            if r_nxt < 0
                || c_nxt < 0
                || r_nxt as usize >= len_r
                || c_nxt as usize >= len_c
                || visited[r_nxt as usize][c_nxt as usize]
                || grid[r_nxt as usize][c_nxt as usize] == Self::WATER
            {
                continue;
            }
            Self::dfs((r_nxt as usize, c_nxt as usize), visited, grid, queue);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let actual = Solution::shortest_bridge(grid);
        let expected = 1;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1, 0], vec![0, 0, 0], vec![0, 0, 1]];
        let actual = Solution::shortest_bridge(grid);
        let expected = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let grid: Vec<Vec<i32>> = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ];
        let actual = Solution::shortest_bridge(grid);
        let expected = 1;
        assert_eq!(expected, actual);
    }
}
