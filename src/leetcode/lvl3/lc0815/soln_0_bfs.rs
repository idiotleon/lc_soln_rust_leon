use std::collections::{HashMap, HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/bus-routes/
/// Time Complexity:    O(V + E) ~ O(`len_vts` + `len_rs`)
/// Space Complexity:   O(V + E) ~ O(`len_vts` + `len_rs`)
/// Reference:
/// https://leetcode.com/problems/bus-routes/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let len_vts: usize = routes.iter().map(|route| route.len()).sum();
        let len_rs: usize = routes.len();
        let graph: HashMap<i32, Vec<i32>> = Self::build_graph(len_vts, &routes);
        let (mut queue, mut seen): (VecDeque<i32>, HashSet<i32>) = {
            let mut queue: VecDeque<i32> = VecDeque::with_capacity(len_rs);
            let mut seen: HashSet<i32> = HashSet::with_capacity(len_rs);
            if let Some(stops) = graph.get(&source) {
                for &stop in stops {
                    queue.push_back(stop);
                    seen.insert(stop);
                }
            }
            (queue, seen)
        };
        let mut cnt: i32 = 1;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    for &stop in &routes[cur as usize] {
                        if stop == target {
                            return cnt;
                        }
                        if let Some(nxts) = graph.get(&stop) {
                            for &nxt in nxts {
                                if seen.insert(nxt) {
                                    queue.push_back(nxt);
                                }
                            }
                        }
                    }
                }
            }
            cnt += 1;
        }
        return -1;
    }
    fn build_graph(len_vts: usize, routes: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let len_rs: usize = routes.len();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_vts);
        for idx in 0..len_rs {
            for &stop in &routes[idx] {
                graph.entry(stop).or_default().push(idx as i32);
            }
        }
        return graph;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let routes: Vec<Vec<i32>> = vec![vec![1, 2, 7], vec![3, 6, 7]];
        let source: i32 = 1;
        let target: i32 = 6;
        let expected: i32 = 2;
        let actual: i32 = Solution::num_buses_to_destination(routes, source, target);
        assert_eq!(expected, actual);
    }
}
