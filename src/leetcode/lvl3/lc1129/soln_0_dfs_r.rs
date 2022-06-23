/// @author: Leon
/// https://leetcode.com/problems/shortest-path-with-alternating-colors/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    const COLOR_RED: u8 = 0;
    const COLOR_BLUE: u8 = 1;
    const NOT_EXIST: i32 = -1;
    const RANGE: i32 = 100 + 7;
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n: usize = n as usize;
        let mut paths: Vec<Vec<i32>> = {
            let mut paths: Vec<Vec<i32>> = vec![vec![Self::RANGE; n]; 2];
            paths[0][0] = 0;
            paths[1][0] = 0;
            paths
        };
        Self::dfs(0, Self::COLOR_RED, 0, &red_edges, &blue_edges, &mut paths);
        Self::dfs(0, Self::COLOR_BLUE, 0, &red_edges, &blue_edges, &mut paths);
        for idx in 1..n {
            let shorter = std::cmp::min(
                paths[Self::COLOR_RED as usize][idx],
                paths[Self::COLOR_BLUE as usize][idx],
            );
            paths[0][idx] = if shorter == Self::RANGE {
                Self::NOT_EXIST
            } else {
                shorter
            }
        }
        paths[0].to_vec()
    }
    fn dfs(
        cur: i32,
        cur_color: u8,
        steps: i32,
        red_edges: &Vec<Vec<i32>>,
        blue_edges: &Vec<Vec<i32>>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let edges = if cur_color == Self::COLOR_RED {
            red_edges
        } else {
            blue_edges
        };
        let other_color = if cur_color == Self::COLOR_RED {
            Self::COLOR_BLUE
        } else {
            Self::COLOR_RED
        };
        for edge in edges {
            let from = edge[0];
            let to = edge[1];
            if from == cur && paths[other_color as usize][to as usize] > 1 + steps {
                paths[other_color as usize][to as usize] = 1 + steps;
                Self::dfs(to, other_color, 1 + steps, red_edges, blue_edges, paths);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_with_sample_input_1_should_return_expected() {
        let n: i32 = 3;
        let red_edges: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 2]];
        let blue_edges: Vec<Vec<i32>> = vec![];
        let actual = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
        let expected: Vec<i32> = vec![0, 1, -1];
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_with_sample_input_2_should_return_expected() {
        let n: i32 = 3;
        let red_edges: Vec<Vec<i32>> = vec![vec![0, 1]];
        let blue_edges: Vec<Vec<i32>> = vec![vec![2, 1]];
        let actual = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
        let expected: Vec<i32> = vec![0, 1, -1];
        assert_eq!(expected, actual);
    }
}
