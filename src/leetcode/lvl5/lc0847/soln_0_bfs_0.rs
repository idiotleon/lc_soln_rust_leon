use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/shortest-path-visiting-all-nodes/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
/// Reference:
/// https://leetcode.com/problems/shortest-path-visiting-all-nodes/discuss/135809/Fast-BFS-Solution-(46ms)-Clear-Detailed-Explanation-Included/237938
/// https://leetcode.com/problems/shortest-path-visiting-all-nodes/discuss/135809/Fast-BFS-Solution-(46ms)-Clear-Detailed-Explanation-Included
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let len_n: usize = graph.len();
        let full_mask: u16 = (1 << len_n) - 1;
        let mut seen: HashSet<Rc<State>> = HashSet::new();
        let mut queue: VecDeque<Rc<State>> = {
            let mut queue: VecDeque<Rc<State>> = VecDeque::new();
            for idx in 0..len_n {
                let state = Rc::new(State {
                    node: idx as u8,
                    bit_mask: 1 << idx,
                });
                queue.push_back(state.clone());
                seen.insert(state.clone());
            }
            queue
        };
        let mut lvl: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(state) = queue.pop_front() {
                    if state.clone().bit_mask == full_mask {
                        return lvl;
                    }
                    for &nxt in graph[state.clone().node as usize].iter() {
                        let nxt_state = Rc::new(State {
                            node: nxt as u8,
                            bit_mask: state.clone().bit_mask | (1 << nxt),
                        });
                        if seen.insert(nxt_state.clone()) {
                            queue.push_back(nxt_state.clone());
                        };
                    }
                }
            }
            lvl += 1;
        }
        lvl
    }
}

// #[derive(Eq, Hash, PartialEq)]
struct State {
    node: u8,
    bit_mask: u16,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.node.hash(state);
        self.bit_mask.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.bit_mask == other.bit_mask
    }
}

impl Eq for State {}
