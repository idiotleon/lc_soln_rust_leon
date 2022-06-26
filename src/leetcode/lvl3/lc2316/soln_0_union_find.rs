use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let n: usize = n as usize;
        let _len_es: usize = edges.len();
        let mut roots: Vec<usize> = vec![0; n]
            .into_iter()
            .enumerate()
            .map(|(idx, _)| idx)
            .collect();
        let mut ranks: Vec<u16> = vec![1; n];
        for edge in edges {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;
            Self::union(u, v, &mut roots, &mut ranks);
        }
        Self::get_count(&mut roots)
    }
    fn get_count(roots: &mut Vec<usize>) -> i64 {
        let freqs: Vec<i64> = {
            let mut root_to_freq: HashMap<usize, i64> = HashMap::new();
            let len_rts: usize = roots.len();
            for idx in 0..len_rts {
                *root_to_freq
                    .entry(Self::find(roots[idx], roots))
                    .or_default() += 1;
            }
            root_to_freq.into_values().collect()
        };
        let len_fs: usize = freqs.len();
        let mut cnt: i64 = 0;
        for lo in 0..len_fs - 1 {
            for hi in lo + 1..len_fs {
                cnt += freqs[lo] * freqs[hi];
            }
        }
        cnt
    }
    fn union(u: usize, v: usize, roots: &mut Vec<usize>, ranks: &mut Vec<u16>) {
        let root_u: usize = Self::find(u, roots);
        let root_v: usize = Self::find_r(v, roots);
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
    }
    fn find(u: usize, roots: &mut Vec<usize>) -> usize {
        let mut cur = u;
        while cur != roots[cur] {
            roots[cur] = roots[roots[cur]];
            cur = roots[cur];
        }
        cur
    }
    fn find_r(u: usize, roots: &mut Vec<usize>) -> usize {
        if roots[u] != u {
            roots[u] = Self::find_r(roots[u], roots);
            return roots[u];
        }
        roots[u]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_with_test_case_45_should_return_expected() {
        let n: i32 = 11;
        let edges: Vec<Vec<i32>> = vec![
            vec![5, 0],
            vec![1, 0],
            vec![10, 7],
            vec![9, 8],
            vec![7, 2],
            vec![1, 3],
            vec![0, 2],
            vec![8, 5],
            vec![4, 6],
            vec![4, 2],
        ];
        let actual: i64 = Solution::count_pairs(n, edges);
        let expected: i64 = 0;
        assert_eq!(expected, actual);
    }
}
