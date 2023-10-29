/// @author: Leon
/// https://leetcode.com/problems/shortest-path-in-binary-matrix/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [(isize, isize)] = &[
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        let mut paths: Vec<Vec<(usize, usize)>> = Vec::new();
        Self::dfs((0, 0), &mut visited, &grid, &mut Vec::new(), &mut paths);
        return paths
            .into_iter()
            .map(|list| list.len() as i32)
            .min()
            .unwrap_or(-1);
    }
    fn dfs(
        coord: (isize, isize),
        visited: &mut Vec<Vec<bool>>,
        grid: &Vec<Vec<i32>>,
        path: &mut Vec<(usize, usize)>,
        paths: &mut Vec<Vec<(usize, usize)>>,
    ) {
        let (r, c) = coord;
        if r < 0 || c < 0 {
            return;
        }
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let r: usize = r as usize;
        let c: usize = c as usize;
        if r >= len_rs || c >= len_cs || grid[r][c] != 0 || visited[r][c] {
            return;
        }
        if r == len_rs - 1 && c == len_cs - 1 {
            path.push((r, c));
            paths.push(path.to_vec());
            return;
        }
        visited[r][c] = true;
        path.push((r, c));
        for (r_delta, c_delta) in Self::DIRS {
            let r_nxt: isize = r as isize + r_delta;
            let c_nxt: isize = c as isize + c_delta;
            Self::dfs((r_nxt, c_nxt), visited, grid, path, paths);
        }
        visited[r][c] = false;
        path.pop();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let expected: i32 = 2;
        let actual = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let expected: i32 = 4;
        let actual = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let expected: i32 = -1;
        let actual = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(expected, actual);
    }
}
