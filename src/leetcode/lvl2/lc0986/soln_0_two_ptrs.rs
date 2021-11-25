/// https://leetcode.com/problems/interval-list-intersections/
/// Time Complexity:    O(min(`len1`, `len2`))
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len1 = first_list.len();
        let len2 = second_list.len();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        while idx1 < len1 && idx2 < len2{
            let start = std::cmp::max(first_list[idx1][0], second_list[idx2][0]);
            let end = std::cmp::min(first_list[idx1][1], second_list[idx2][1]);
            if start <= end{
                ans.push(vec![start, end]);
            }
            if first_list[idx1][1] < second_list[idx2][1]{
                idx1 += 1;
            }else{
                idx2 += 1;
            }
        }
        ans
    }
}