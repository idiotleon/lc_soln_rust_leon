// @author: Leon
// https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/
// Time Complexity:     O(`n`)
// Space Complexity:    O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut roots = vec![0; n];
        let mut ranks = vec![0; n];
        for i in 0..n {
            roots[i] = i;
            ranks[i] = 1;
        }
        let mut count = n;
        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;

            Self::union(u, v, &mut roots, &mut ranks, &mut count)
        }
        return count as i32;
    }
    fn find(x: usize, roots: &mut Vec<usize>) -> usize {
        let mut x = x;
        while x != roots[x] {
            roots[x] = roots[roots[x]];
            x = roots[x];
        }
        return x;
    }
    fn union(
        x: usize,
        y: usize,
        mut roots: &mut Vec<usize>,
        ranks: &mut Vec<usize>,
        count: &mut usize,
    ) {
        let root_x = Self::find(x, &mut roots);
        let root_y = Self::find(y, &mut roots);
        if root_x == root_y {
            return;
        }
        if ranks[root_x] > ranks[root_y] {
            roots[root_y] = root_x;
            ranks[root_x] += 1;
        } else {
            roots[root_x] = root_y;
            ranks[root_y] += 1;
        }
        *count -= 1;
    }
}
