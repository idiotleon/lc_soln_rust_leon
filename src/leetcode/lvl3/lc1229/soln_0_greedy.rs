// https://leetcode.com/problems/meeting-scheduler/
//
// Time Complexity:     O(`lens1` * lg(`lens1`)) + O(`lens2` * lg(`lens2`)) ~ O(lens * lg(lens)))
// Space Complexity:    O(1)
//
// Reference:
//  https://leetcode.com/problems/meeting-scheduler/discuss/408491/JavaC%2B%2B-Two-Pointer-Clean-code-O(NlogN)-for-Sorting-Beat-100
use std::cmp;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_available_duration(
        slots1: Vec<Vec<i32>>,
        slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        let lens1 = slots1.len();
        let lens2 = slots2.len();
        let mut slots1 = slots1;
        let mut slots2 = slots2;
        slots1.sort();
        slots2.sort();
        let mut idx1 = 0;
        let mut idx2 = 0;

        while idx1 < lens1 && idx2 < lens2 {
            let start_intersect = cmp::max(slots1[idx1][0], slots2[idx2][0]);
            let end_intersect = cmp::min(slots1[idx1][1], slots2[idx2][1]);

            if start_intersect + duration <= end_intersect {
                return vec![start_intersect, start_intersect + duration];
            } else if slots1[idx1][1] < slots2[idx2][1] {
                idx1 += 1;
            } else {
                idx2 += 1;
            }
        }
        return vec![];
    }
}
