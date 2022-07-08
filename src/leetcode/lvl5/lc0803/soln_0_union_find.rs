/// @author: Leon
/// https://leetcode.com/problems/bricks-falling-when-hit/
/// Time Complexity:    O(amortized(`len_rs` * `len_cs`))
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/bricks-falling-when-hit/discuss/195781/Union-find-Logical-Thinking
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let len_ns: usize = len_rs * len_cs;
        let len_hs: usize = hits.len();
        let mut grid: Vec<Vec<i32>> = {
            let mut grid = grid;
            for hit in &hits {
                let r: usize = hit[0] as usize;
                let c: usize = hit[1] as usize;
                if grid[r][c] == 1 {
                    grid[r][c] = 2;
                }
            }
            grid
        };
        let mut roots: Vec<usize> = (0..len_ns + 1).collect();
        let mut sizes: Vec<i32> = vec![1; len_ns + 1];
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == 1 {
                    Self::union_around((r, c), &grid, &mut roots, &mut sizes);
                }
            }
        }
        let mut bricks_left = sizes[Self::find(0, &mut roots)];
        let mut bricks_dropped: Vec<i32> = vec![0; len_hs];
        for (idx, hit) in hits.iter().enumerate().rev() {
            let r: usize = hit[0] as usize;
            let c: usize = hit[1] as usize;
            if grid[r][c] == 2 {
                grid[r][c] = 1;
                Self::union_around((r, c), &grid, &mut roots, &mut sizes);
                let cur_bricks_left = sizes[Self::find(0, &mut roots)];
                bricks_dropped[idx] = std::cmp::max(cur_bricks_left - bricks_left - 1, 0) as i32;
                bricks_left = cur_bricks_left;
            }
        }
        bricks_dropped
    }
    fn union_around(
        coord: (usize, usize),
        grid: &Vec<Vec<i32>>,
        roots: &mut Vec<usize>,
        sizes: &mut Vec<i32>,
    ) {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let (r, c) = coord;
        for d in 0..4 {
            let r_nxt: isize = r as isize + Self::DIRS[d];
            let c_nxt: isize = c as isize + Self::DIRS[d + 1];
            if r_nxt < 0
                || c_nxt < 0
                || r_nxt as usize >= len_rs
                || c_nxt as usize >= len_cs
                || grid[r_nxt as usize][c_nxt as usize] != 1
            {
                continue;
            }
            Self::union(
                Self::hash(r, c, len_cs),
                Self::hash(r_nxt as usize, c_nxt as usize, len_cs),
                roots,
                sizes,
            );
        }
        if r == 0 {
            Self::union(0, Self::hash(r, c, len_cs), roots, sizes);
        }
    }
    fn union(x: usize, y: usize, roots: &mut Vec<usize>, sizes: &mut Vec<i32>) {
        let root_x: usize = Self::find(x, roots);
        let root_y: usize = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        if sizes[root_x] > sizes[root_y] {
            roots[root_y] = root_x;
            sizes[root_x] += sizes[root_y];
            sizes[root_y] = 0;
        } else {
            roots[root_x] = root_y;
            sizes[root_y] += sizes[root_x];
            sizes[root_x] = 0;
        }
    }
    fn find(mut x: usize, roots: &mut Vec<usize>) -> usize {
        while x != roots[x] {
            roots[x] = roots[roots[x]];
            x = roots[x];
        }
        x
    }
    fn hash(r: usize, c: usize, len_cs: usize) -> usize {
        r * len_cs + c + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expted() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]];
        let hits: Vec<Vec<i32>> = vec![vec![1, 0]];
        let expected: Vec<i32> = vec![2];
        let actual = Solution::hit_bricks(grid, hits);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expted() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]];
        let hits: Vec<Vec<i32>> = vec![vec![1, 1], vec![1, 0]];
        let expected: Vec<i32> = vec![0, 0];
        let actual = Solution::hit_bricks(grid, hits);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_34_should_return_expted() {
        let grid: Vec<Vec<i32>> = vec![vec![1], vec![1], vec![1], vec![1], vec![1]];
        let hits: Vec<Vec<i32>> = vec![vec![3, 0], vec![4, 0], vec![1, 0], vec![2, 0], vec![0, 0]];
        let expected: Vec<i32> = vec![1, 0, 1, 0, 0];
        let actual = Solution::hit_bricks(grid, hits);
        assert_eq!(expected, actual);
    }
}
