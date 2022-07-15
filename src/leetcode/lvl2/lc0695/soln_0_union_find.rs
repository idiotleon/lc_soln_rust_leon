/// @author: Leon
/// https://leetcode.com/problems/max-area-of-island/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let len = len_rs * len_cs;
        const LAND: i32 = 1;
        const WATER: i32 = 0;
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        let mut roots: Vec<i32> = {
            let mut roots = vec![0; len];
            for idx in 0..len {
                roots[idx] = idx as i32;
            }
            roots
        };
        let mut sizes: Vec<i32> = vec![1; len];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        let mut at_least_one: bool = false;
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == WATER || visited[r][c] {
                    continue;
                }
                at_least_one = true;
                visited[r][c] = true;
                for d in 0..4 {
                    let r_nxt: isize = r as isize + DIRS[d];
                    let c_nxt: isize = c as isize + DIRS[d + 1];
                    if r_nxt < 0
                        || c_nxt < 0
                        || r_nxt as usize >= len_rs
                        || c_nxt as usize >= len_cs
                    {
                        continue;
                    }
                    let r_nxt: usize = r_nxt as usize;
                    let c_nxt: usize = c_nxt as usize;
                    if grid[r_nxt][c_nxt] == WATER {
                        continue;
                    }
                    Self::union(
                        r * len_cs + c,
                        r_nxt * len_cs + c_nxt,
                        &mut roots,
                        &mut sizes,
                    );
                }
            }
        }
        if at_least_one {
            sizes.into_iter().max().unwrap()
        } else {
            0
        }
    }
    fn find(x: usize, roots: &mut Vec<i32>) -> usize {
        if roots[x] != x as i32 {
            roots[x] = Self::find(roots[x] as usize, roots) as i32;
        }
        roots[x] as usize
    }
    fn union(x: usize, y: usize, roots: &mut Vec<i32>, sizes: &mut Vec<i32>) {
        let root_x = Self::find(x, roots);
        let root_y = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        if sizes[root_x] > sizes[root_y] {
            roots[root_y] = root_x as i32;
            sizes[root_x] += sizes[root_y];
            sizes[root_y] = 0;
        } else {
            roots[root_x] = root_y as i32;
            sizes[root_y] += sizes[root_x];
            sizes[root_x] = 0;
        }
    }
}
