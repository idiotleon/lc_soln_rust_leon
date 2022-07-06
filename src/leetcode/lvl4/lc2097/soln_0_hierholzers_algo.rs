use std::collections::{HashMap, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/valid-arrangement-of-pairs/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/valid-arrangement-of-pairs/discuss/1611978/C++-Eulerian-Path-with-Explanation/1180974
/// https://leetcode.com/problems/valid-arrangement-of-pairs/discuss/1611978/C%2B%2B-Eulerian-Path-with-Explanation
/// Note:
/// this is not yet a correct solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let _len_ps: usize = pairs.len();
        let (mut graph, out) = Self::build_graph(&pairs);
        let start = out
            .keys()
            .filter(|&k| *out.get(k).unwrap() == 1)
            .nth(0)
            .unwrap_or(&pairs[0][0]);
        // let (&start, _) = out
        //     .iter()
        //     .filter(|(_, &v)| v == 1)
        //     .nth(0)
        //     .unwrap_or((&pairs[0][0], &0));
        let mut ans: VecDeque<Vec<i32>> = VecDeque::new();
        Self::dfs(*start, &mut graph, &mut ans);
        ans.into_iter().collect()
    }
    fn dfs(u: i32, graph: &mut HashMap<i32, VecDeque<i32>>, res: &mut VecDeque<Vec<i32>>) {
        if let Some(stk) = graph.get_mut(&u) {
            if let Some(v) = stk.pop_back() {
                Self::dfs(v, graph, res);
                res.push_front(vec![u, v]);
            }
        }
    }
    fn build_graph(pairs: &Vec<Vec<i32>>) -> (HashMap<i32, VecDeque<i32>>, HashMap<i32, i32>) {
        let mut graph: HashMap<i32, VecDeque<i32>> = HashMap::new();
        let mut out: HashMap<i32, i32> = HashMap::new();
        for p in pairs {
            let u: i32 = p[0];
            let v: i32 = p[1];
            graph.entry(u).or_default().push_back(v);
            *out.entry(u).or_default() += 1;
            *out.entry(v).or_default() -= 1;
        }
        (graph, out)
    }
}
