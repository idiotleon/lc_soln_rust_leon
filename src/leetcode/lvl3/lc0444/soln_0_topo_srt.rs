use std::collections::{HashMap, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/sequence-reconstruction/
/// Time Complexity:    O(V + E) ~ O(`len_ns` + `_len_ss`)
/// Space Complexity:   O(V + E) ~ O(`len_ns` + `_len_ss`)
/// Reference:
/// https://leetcode.com/problems/sequence-reconstruction/discuss/92580/Java-Solution-using-BFS-Topological-Sort/143050
/// https://leetcode.com/problems/sequence-reconstruction/discuss/92580/Java-Solution-using-BFS-Topological-Sort/97078
/// https://leetcode.com/problems/sequence-reconstruction/discuss/92580/Java-Solution-using-BFS-Topological-Sort
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        let len_ns: usize = nums.len();
        let _len_ss: usize = sequences.len();
        let (graph, mut counts) = Self::build_graph(len_ns, sequences);
        let mut queue: VecDeque<i32> = {
            let mut queue: VecDeque<i32> = VecDeque::with_capacity(len_ns);
            for (&idx, &count) in counts.iter() {
                if count == 0 {
                    queue.push_back(idx);
                }
            }
            queue
        };
        let mut idx: usize = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            // the topological sort is expected to be unique
            if len_q > 1 {
                return false;
            }
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    // the topological sort order is expected to match the order
                    if nums[idx] != cur {
                        return false;
                    }
                    idx += 1;
                    if let Some(nxts) = graph.get(&cur) {
                        for &nxt in nxts {
                            if let Some(cnt) = counts.get_mut(&nxt) {
                                *cnt -= 1;
                                if *cnt == 0 {
                                    queue.push_back(nxt);
                                }
                            }
                        }
                    }
                }
            }
        }
        // the topological sort is expected to traverse ll the elements
        return idx == len_ns;
    }
    fn build_graph(
        n: usize,
        sequences: Vec<Vec<i32>>,
    ) -> (HashMap<i32, Vec<i32>>, HashMap<i32, u16>) {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(n + 1);
        let mut counts: HashMap<i32, u16> = HashMap::with_capacity(n + 1);
        for seq in sequences {
            let len_s: usize = seq.len();
            for idx in 0..len_s {
                graph.entry(seq[idx]).or_default();
                counts.entry(seq[idx]).or_default();
                if idx > 0 {
                    graph.entry(seq[idx - 1]).or_default().push(seq[idx]);
                    *counts.entry(seq[idx]).or_default() += 1;
                }
            }
        }
        return (graph, counts);
    }
}
