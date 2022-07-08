/// @author: Leon
/// https://leetcode.com/problems/connecting-cities-with-minimum-cost/
/// Time Complexity:    O(V * lg(E)) ~ O(`n` * lg(`_len_cs`))
/// Space Complexity:   O(V) ~ O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let _len_cs: usize = connections.len();
        let n: usize = n as usize;
        let conn: Vec<Vec<i32>> = {
            let mut conn = connections;
            conn.sort_by_key(|e| e[2]);
            conn
        };
        let mut roots: Vec<usize> = {
            let mut roots = vec![0; n];
            for idx in 0..n {
                roots[idx] = idx;
            }
            roots
        };
        let mut ranks: Vec<u16> = vec![1; n];
        let mut isolated: u16 = n as u16;
        let mut ans: i32 = 0;
        for c in conn {
            let u: usize = c[0] as usize - 1;
            let v: usize = c[1] as usize - 1;
            let cost: i32 = c[2];
            if Self::find(u, &mut roots) != Self::find(v, &mut roots) {
                ans += cost;
                Self::union(u, v, &mut roots, &mut ranks, &mut isolated);
            }
        }
        if isolated == 1 {
            ans
        } else {
            -1
        }
    }
    fn union(u: usize, v: usize, roots: &mut Vec<usize>, ranks: &mut Vec<u16>, isolated: &mut u16) {
        let root_u: usize = Self::find(u, roots);
        let root_v: usize = Self::find(v, roots);
        if root_u == root_v {
            return;
        }
        if ranks[root_u] > ranks[root_v] {
            roots[root_v] = root_u;
            ranks[root_u] += 1;
        } else {
            roots[root_u] = root_v;
            ranks[root_v] += 1;
        }
        *isolated -= 1;
    }
    fn find(mut x: usize, roots: &mut Vec<usize>) -> usize {
        while x != roots[x] {
            roots[x] = roots[roots[x]];
            x = roots[x];
        }
        x
    }
}
