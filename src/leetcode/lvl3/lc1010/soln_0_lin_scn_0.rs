/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
///
/// Time Complexity:    O(`DURATION`)
/// Space Complexity:   O(`DURATION`) ~ O(1)
///
/// Reference:
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/discuss/256738/JavaC++Python-Two-Sum-with-K-60/250324
/// https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/discuss/256738/JavaC%2B%2BPython-Two-Sum-with-K-60
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_pairs_divisible_by60(times: Vec<i32>) -> i32 {
        const DURATION: usize = 60;

        let mut types = vec![0; DURATION];

        for &time in times.iter() {
            let hash = (time as usize) % DURATION;
            types[hash] += 1;
        }

        let mut cnt = 0;

        if types[0] > 0 {
            cnt += types[0] * (types[0] - 1) / 2;
        }

        if types[30] > 0 {
            cnt += types[30] * (types[30] - 1) / 2;
        }

        for i in 1..30 {
            cnt += types[i] * types[DURATION - i];
        }

        cnt
    }
}
