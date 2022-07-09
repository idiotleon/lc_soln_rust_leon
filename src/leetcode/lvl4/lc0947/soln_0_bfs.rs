use std::collections::{HashMap, HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
/// Time Complexity:    O(V + E) ~ O()
/// Space Complexity:   O(V + E) ~ O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let len_sts: usize = stones.len();
        let graph = Self::build_graph(&stones);
        let mut seen: HashSet<i32> = HashSet::with_capacity(len_sts);
        let mut cnt: u16 = 0;
        for stone in stones {
            for idx in 0..2 {
                let u = if idx == 0 { stone[0] } else { !stone[1] };
                if seen.insert(u) {
                    cnt += 1;
                    Self::bfs(u, &mut seen, &graph);
                }
            }
        }
        len_sts as i32 - cnt as i32
    }
    fn bfs(cur: i32, seen: &mut HashSet<i32>, graph: &HashMap<i32, Vec<i32>>) {
        let len_sts: usize = graph.len();
        let mut queue: VecDeque<i32> = {
            let mut queue = VecDeque::with_capacity(len_sts);
            queue.push_back(cur);
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if let Some(nxts) = graph.get(&cur) {
                        for &nxt in nxts {
                            if seen.insert(nxt) {
                                queue.push_back(nxt);
                            }
                        }
                    }
                }
            }
        }
    }
    fn build_graph(stones: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let len_sts: usize = stones.len();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_sts);
        for stone in stones {
            let u = stone[0];
            let v = !stone[1];
            graph.entry(u).or_default().push(v);
            graph.entry(v).or_default().push(u);
        }
        graph
    }
}
