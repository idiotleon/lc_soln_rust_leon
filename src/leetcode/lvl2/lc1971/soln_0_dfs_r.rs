/// @author: Leon
/// https://leetcode.com/problems/find-if-path-exists-in-graph/
///
/// Time Complexity:    O(V + E) ~ O(`n` + `n_edges`)
/// Space Complexity:   O(V + E) ~ O(`n` + `n_edges`)
use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        // not used
        // let n_edges = edges.len();
        let graph: Vec<HashSet<usize>> = {
            let mut tmp: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
            for edge in edges.iter(){
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                tmp[u].insert(v);
                tmp[v].insert(u);
            }
            tmp
        };
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(start as usize);
        Self::dfs(&mut seen, start as usize, end as usize, &graph)
    }
    fn dfs(seen: &mut HashSet<usize>, cur: usize, end: usize, graph: &Vec<HashSet<usize>>) -> bool{
        if cur == end{
            return true;
        }
        for &next in &graph[cur]{
            if seen.insert(next){
                if Self::dfs(seen, next, end, graph){
                    return true;
                }
            }
        }
        false
    }
}