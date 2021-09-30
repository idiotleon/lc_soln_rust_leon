/// https://leetcode.com/problems/find-smallest-common-element-in-all-rows/
///
/// Time Complexity:    O(`num_rows` * num_cols)
/// Space Complexity:   O(`RANGE`)
///
/// Reference:
///     https://leetcode.com/problems/find-smallest-common-element-in-all-rows/discuss/1122045/Rust-simple-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_common_element(mat: Vec<Vec<i32>>) -> i32 {
        const RANGE: usize = 10007;
        let num_rows = mat.len();
        let mut count = vec![0; RANGE];
        for row in mat {
            for num in row {
                count[num as usize - 1] += 1;
            }
        }

        for i in 0..RANGE {
            if count[i] == num_rows {
                return i as i32 + 1;
            }
        }

        -1
    }
}
