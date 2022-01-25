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
        for path in &paths {
            let dest = &path[1];
            // this logic largely exploits the restrictions of the data input
            if !graph.contains_key(dest) {
                return dest.to_owned();
            }
        }
        unreachable!()
    }
    fn build_graph(paths: &Vec<Vec<String>>) -> HashMap<String, String> {
        let mut graph: HashMap<String, String> = HashMap::new();
        for path in paths {
            let src = &path[0];
            let dest = &path[1];
            graph.insert(src.to_owned(), dest.to_owned());
        }
        graph
    }
}
