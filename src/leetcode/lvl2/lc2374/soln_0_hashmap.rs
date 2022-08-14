use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/node-with-highest-edge-score/
/// Time Complexity:    O(`len_es`)
/// Space Complexity:   O(`len_es`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn edge_score(edges: Vec<i32>) -> i32 {
        let len_es: usize = edges.len();
        let vtx_to_score: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(len_es);
            for (score, vtx) in edges.into_iter().enumerate() {
                *map.entry(vtx).or_default() += score as i32;
            }
            map
        };
        let mut ans: i32 = len_es as i32;
        let mut max: i32 = -1;
        for (vtx, score) in vtx_to_score.into_iter() {
            if score > max {
                ans = vtx;
                max = score;
            } else if score == max {
                if ans > vtx {
                    ans = vtx;
                }
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let edges: Vec<i32> = vec![1, 0, 0, 0, 0, 7, 7, 5];
        let expected: i32 = 7;
        let actual = Solution::edge_score(edges);
        assert_eq!(expected, actual);
    }
}
