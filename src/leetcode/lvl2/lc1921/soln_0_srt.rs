/// @author: Leon
/// https://leetcode.com/problems/eliminate-maximum-number-of-monsters/
/// Time Complexity:    O(`len_ms` * lg(`len_ms`))
/// Space Complexity:   O(`len_ms`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let len_ms: usize = dist.len();
        // sorted arrival timestamps
        let sorted_ts: Vec<f32> = {
            let mut res: Vec<f32> = vec![0_f32; len_ms];
            for idx in 0..len_ms {
                res[idx] = dist[idx] as f32 / speed[idx] as f32;
            }
            res.sort_by(|a, b| a.partial_cmp(b).unwrap());
            res
        };
        let mut cnt: i32 = 0;
        for idx in 0..len_ms {
            if sorted_ts[idx] <= idx as f32 {
                break;
            }
            cnt += 1;
        }
        return cnt;
    }
}
