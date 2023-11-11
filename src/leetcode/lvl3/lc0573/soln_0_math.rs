/// @author: Leon
/// https://leetcode.com/problems/squirrel-simulation/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_distance(
        _height: i32,
        _width: i32,
        tree: Vec<i32>,
        squirrel: Vec<i32>,
        nuts: Vec<Vec<i32>>,
    ) -> i32 {
        const RANGE: i32 = 100 * 5000;
        const IMPS: i32 = RANGE + 7;
        let _len_ns: usize = nuts.len();
        let mut ans: i32 = IMPS;
        let total_distance: i32 = Self::get_total_distance(&tree, &nuts);
        for nut in &nuts {
            let dist_sq: i32 = Self::get_distance(&squirrel, nut);
            let dist_selected: i32 = Self::get_distance(&tree, nut);
            let res = total_distance - dist_selected + dist_sq;
            ans = std::cmp::min(ans, res);
        }
        return ans;
    }
    fn get_total_distance(tree: &Vec<i32>, nuts: &Vec<Vec<i32>>) -> i32 {
        let mut sum: i32 = 0;
        for nut in nuts {
            sum += Self::get_distance(tree, nut);
        }
        return sum * 2;
    }
    fn get_distance(coord1: &Vec<i32>, coord2: &Vec<i32>) -> i32 {
        return (coord1[0] - coord2[0]).abs() + (coord1[1] - coord2[1]).abs();
    }
}
