/// @author: Leon
/// https://leetcode.com/problems/swim-in-rising-water/
/// Time Complexity:    O(E * lg(E)) or O(E * lg(V)) ~
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/swim-in-rising-water/discuss/118204/Java-DFS-and-Union-Find/752605
/// https://leetcode.com/problems/swim-in-rising-water/discuss/118204/Java-DFS-and-Union-Find/309165
/// https://leetcode.com/problems/swim-in-rising-water/discuss/118204/Java-DFS-and-Union-Find
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let len_ns: usize = len_rs * len_cs;
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        let hashes: Vec<usize> = {
            let mut hashes: Vec<usize> = vec![0; len_rs * len_cs];
            for r in 0..len_rs {
                for c in 0..len_cs {
                    hashes[grid[r][c] as usize] = Self::hash(r, c, len_cs);
                }
            }
            hashes
        };
        let mut roots: Vec<usize> = {
            let mut roots = vec![0; len_ns];
            for idx in 0..len_ns {
                roots[idx] = idx;
            }
            roots
        };
        let mut ranks: Vec<u8> = vec![1; len_ns];
        for time in 0..len_rs * len_cs {
            let idx: usize = hashes[time];
            let r: usize = idx / len_cs;
            let c: usize = idx % len_cs;
            for d in 0..4 {
                let r_nxt: isize = r as isize + DIRS[d];
                let c_nxt: isize = c as isize + DIRS[d + 1];
                if r_nxt < 0
                    || c_nxt < 0
                    || r_nxt as usize >= len_rs
                    || c_nxt as usize >= len_cs
                    || grid[r_nxt as usize][c_nxt as usize] > time as i32
                {
                    continue;
                }
                Self::union(
                    Self::hash(r, c, len_cs),
                    Self::hash(r_nxt as usize, c_nxt as usize, len_cs),
                    &mut roots,
                    &mut ranks,
                );
            }
            if Self::is_connected(0, len_ns - 1, &mut roots) {
                return time as i32;
            }
        }
        -1
    }
    fn union(u: usize, v: usize, roots: &mut Vec<usize>, ranks: &mut Vec<u8>) {
        let root_u: usize = Self::find(u, roots);
        let root_v: usize = Self::find(v, roots);
        if root_u == root_v {
            return;
        }
        if ranks[root_u] > ranks[root_v] {
            roots[root_v] = roots[root_u];
            ranks[root_u] += 1;
        } else {
            roots[root_u] = roots[root_v];
            ranks[root_v] += 1;
        }
    }
    fn hash(r: usize, c: usize, len_cs: usize) -> usize {
        r * len_cs + c
    }
    fn find(x: usize, roots: &mut Vec<usize>) -> usize {
        if roots[x] != x {
            roots[x] = Self::find(roots[x], roots);
        }
        roots[x]
    }
    fn is_connected(u: usize, v: usize, roots: &mut Vec<usize>) -> bool {
        Self::find(u, roots) == Self::find(v, roots)
    }
}
