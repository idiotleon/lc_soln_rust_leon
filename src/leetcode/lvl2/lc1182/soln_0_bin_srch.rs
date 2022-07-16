use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/shortest-distance-to-target-color/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/shortest-distance-to-target-color/discuss/384822/Java-binary-search/1443468
/// https://leetcode.com/problems/shortest-distance-to-target-color/discuss/384822/Java-binary-search
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let _len_cs: usize = colors.len();
        let _len_qs: usize = queries.len();
        let color_to_indices: HashMap<i32, Vec<i32>> = {
            let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
            for (idx, color) in colors.into_iter().enumerate() {
                map.entry(color).or_default().push(idx as i32);
            }
            map
        };
        let mut ans: Vec<i32> = Vec::new();
        for query in queries {
            let target_idx = query[0];
            let target_color = query[1];
            if let Some(indices) = color_to_indices.get(&target_color) {
                ans.push(Self::binary_search(target_idx, indices));
            } else {
                ans.push(-1);
            }
        }
        ans
    }
    fn binary_search(target: i32, indices: &Vec<i32>) -> i32 {
        let len_is: isize = indices.len() as isize;
        let mut smallest: i32 = i32::MAX;
        let mut lo: isize = 0;
        let mut hi: isize = len_is - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let diff = (indices[mid as usize] - target).abs();
            if diff < smallest {
                smallest = diff;
            }
            if target > indices[mid as usize] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        smallest
    }
}
