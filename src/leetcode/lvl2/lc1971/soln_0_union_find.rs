/// @author: Leon
/// https://leetcode.com/problems/find-if-path-exists-in-graph/
///
/// Time Complexity:    O(V + E) ~ O(`n` + `n_edges`)
/// Space Complexity:   O(V + E) ~ O(`n` + `n_edges`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        let n: usize = n as usize;
        let mut roots: Vec<usize> = {
            let mut tmp: Vec<usize> = vec![0; n];
            for idx in 0..n {
                tmp[idx] = idx;
            }
            tmp
        };
        let mut ranks: Vec<usize> = vec![1; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            Self::union(u, v, &mut roots, &mut ranks);
        }
        Self::find(start as usize, &mut roots) == Self::find(end as usize, &mut roots)
    }
    fn union(x: usize, y: usize, roots: &mut Vec<usize>, ranks: &mut Vec<usize>) {
        let root_x = Self::find(x, roots);
        let root_y = Self::find(y, roots);
        if root_x == root_y {
            return;
        }
        if ranks[x] > ranks[y] {
            roots[root_y] = root_x;
            ranks[root_x] += 1;
        } else {
            roots[root_x] = root_y;
            ranks[root_y] += 1;
        }
    }
    fn find(x: usize, roots: &mut Vec<usize>) -> usize {
        let mut x = x;
        while x != roots[x] {
            roots[x] = roots[roots[x]];
            x = roots[x];
        }
        x
    }
}
