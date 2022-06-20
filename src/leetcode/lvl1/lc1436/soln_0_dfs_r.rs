use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/destination-city/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let _len_ps: usize = paths.len();
        let graph = Self::build_graph(&paths);
        Self::dfs(&paths[0][0], &graph).to_owned()
    }
    fn dfs<'a>(cur: &'a str, graph: &'a HashMap<&str, &str>) -> &'a str {
        match graph.get(cur) {
            Some(nxt) => Self::dfs(nxt, graph),
            None => cur,
        }
    }
    fn build_graph(paths: &Vec<Vec<String>>) -> HashMap<&str, &str> {
        let mut graph: HashMap<&str, &str> = HashMap::new();
        for path in paths {
            let src = &path[0];
            let dest = &path[1];
            graph.insert(src, dest);
        }
        graph
    }
}
