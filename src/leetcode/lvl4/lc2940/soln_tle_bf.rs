use std::collections::BTreeSet;

/// @author: Leon
/// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// TLEed
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len_hts: usize = heights.len();
        let len_qs: usize = queries.len();
        let indices: Vec<BTreeSet<usize>> = {
            let mut res: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); len_hts];
            for lo in 0..len_hts - 1 {
                for hi in lo + 1..len_hts {
                    if heights[lo] < heights[hi] {
                        res[lo].insert(hi);
                    }
                }
            }
            res
        };
        let mut ans: Vec<i32> = Vec::with_capacity(len_qs);
        for query in queries {
            let q1: usize = query[0] as usize;
            let q2: usize = query[1] as usize;
            if q1 == q2 {
                ans.push(q1 as i32);
            } else {
                let res: i32 = Self::get_res(q1, q2, &indices, &heights);
                ans.push(res);
            }
        }
        return ans;
    }
    fn get_res(q1: usize, q2: usize, indices: &Vec<BTreeSet<usize>>, heights: &Vec<i32>) -> i32 {
        if heights[q1] < heights[q2] {
            if indices[q1].contains(&q2) {
                return q2 as i32;
            }
        } else if heights[q1] > heights[q2] {
            if indices[q2].contains(&q1) {
                return q1 as i32;
            }
        }
        return Self::find_common(&indices[q1], &indices[q2]);
    }
    fn find_common(indices1: &BTreeSet<usize>, indices2: &BTreeSet<usize>) -> i32 {
        let len1: usize = indices1.len();
        let len2: usize = indices2.len();
        if len1 > len2 {
            return Self::find_common(indices2, indices1);
        }
        for &idx in indices1 {
            if indices2.contains(&idx) {
                return idx as i32;
            }
        }
        return -1;
    }
}
