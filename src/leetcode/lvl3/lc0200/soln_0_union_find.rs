/// @author: Leon
/// https://leetcode.com/problems/number-of-islands/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(1)
///
/// this is a NOT yet correct solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let len = len_rs * len_cs;
        const LAND: char = '1';
        const WATER: char = '0';
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        let mut roots: Vec<i32> = {
            let mut roots = vec![0; len];
            for idx in 0..len {
                roots[idx] = idx as i32;
            }
            roots
        };
        let mut ranks: Vec<i32> = vec![1; len];
        let mut isolated: i32 = len as i32;
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == WATER {
                    continue;
                }
                grid[r][c] = WATER;
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
                        &mut ranks,
                        &mut isolated,
                    );
                }
            }
        }
        isolated
    }
    fn find(x: usize, roots: &mut Vec<i32>) -> usize {
        if roots[x] != x as i32 {
            roots[x] = Self::find(roots[x] as usize, roots) as i32;
        }
        roots[x] as usize
    }
    fn union(x: usize, y: usize, roots: &mut Vec<i32>, ranks: &mut Vec<i32>, isolated: &mut i32) {
        let root_x = Self::find(x, roots);
        let root_y = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        if ranks[root_x] > ranks[root_y] {
            roots[root_y] = root_x as i32;
            ranks[root_x] += 1;
        } else {
            roots[root_x] = root_y as i32;
            ranks[root_y] += 1;
        }
        *isolated -= 1;
    }
}
