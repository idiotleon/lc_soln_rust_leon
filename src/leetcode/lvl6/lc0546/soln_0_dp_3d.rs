/// https://leetcode.com/problems/remove-boxes/
///
/// Time Complexity:    O(`n_boxes` ^ 3)
/// Space ComplexitY:   O(`n_boxes` ^ 3)
///
/// Reference:
/// https://leetcode.com/problems/remove-boxes/discuss/101310/Java-top-down-and-bottom-up-DP-solutions
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n_boxes = boxes.len();
        let mut dp: Vec<Vec<Vec<u16>>> = vec![vec![vec![0; n_boxes]; n_boxes]; n_boxes];
        for j in 0..n_boxes {
            for k in 0..=j {
                dp[j][j][k] = (k as u16 + 1) * (k as u16 + 1);
            }
        }

        for lo in 1..n_boxes {
            for hi in lo..n_boxes {
                let i = hi - lo;

                for k in 0..=i {
                    let res = {
                        let mut tmp = (k as u16 + 1) * (k as u16 + 1) + dp[i + 1][hi][0];

                        for m in i + 1..=hi {
                            if boxes[m] == boxes[i] {
                                tmp = std::cmp::max(tmp, dp[i + 1][m - 1][0] + dp[m][hi][k + 1]);
                            }
                        }

                        tmp
                    };

                    dp[i][hi][k] = res;
                }
            }
        }

        if n_boxes == 0 {
            0
        } else {
            dp[0][n_boxes - 1][0] as i32
        }
    }
}
