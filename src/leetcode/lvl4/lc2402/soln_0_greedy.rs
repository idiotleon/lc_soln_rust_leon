/// @author: Leon
/// https://leetcode.com/problems/meeting-rooms-iii/
/// Time Complexity:    O(`n` * `_len_mts`) + O(`_len_mts` * lg(`_len_mts`))
/// Space Complexity:   O(`n`)
/// Note:
/// this approach takes the advantage of `n` being small
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        const RANGE: i64 = 1e10 as i64 * 5 + 7;
        let _len_mts: usize = meetings.len();
        let n: usize = n as usize;
        let meetings: Vec<Vec<i32>> = {
            let mut mts = meetings;
            mts.sort_by_key(|v| v[0]);
            mts
        };
        let mut end_times: Vec<i64> = vec![0; n];
        let mut freqs: Vec<i32> = vec![0; n];
        for mt in meetings {
            let start = mt[0] as i64;
            let end = mt[1] as i64;
            let duration = end - start;
            let mut arranged: bool = false;
            let mut end_min: i64 = RANGE;
            let mut idx_min: usize = n + 1;
            // to greedily pick the availablity
            for idx in 0..n {
                // if any availability can be found,
                // to pick the one with the lowest index
                if end_times[idx] <= start {
                    end_times[idx] = end;
                    freqs[idx] += 1;
                    arranged = true;
                    break;
                }
                // to keep track of the min value/index along the way
                if end_times[idx] < end_min {
                    end_min = end_times[idx];
                    idx_min = idx;
                }
            }
            // if not any availability can be found,
            // to pick the earliest ending spot
            if !arranged {
                freqs[idx_min] += 1;
                end_times[idx_min] += duration;
            }
        }
        let mut most: i32 = 0;
        let mut ans: usize = 0;
        for (idx, freq) in freqs.into_iter().enumerate() {
            if freq > most {
                most = freq;
                ans = idx;
            }
        }
        return ans as i32;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1_should_return_expected() {
        let n: i32 = 2;
        let meetings: Vec<Vec<i32>> = vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]];
        let expected: i32 = 0;
        let actual = Solution::most_booked(n, meetings);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_sample_input_2_should_return_expected() {
        let n: i32 = 3;
        let meetings: Vec<Vec<i32>> =
            vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]];
        let expected: i32 = 1;
        let actual = Solution::most_booked(n, meetings);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_50_should_return_expected() {
        let n: i32 = 4;
        let meetings: Vec<Vec<i32>> = vec![
            vec![18, 19],
            vec![3, 12],
            vec![17, 19],
            vec![2, 13],
            vec![7, 19],
        ];
        let expected: i32 = 0;
        let actual = Solution::most_booked(n, meetings);
        assert_eq!(expected, actual);
    }
}
