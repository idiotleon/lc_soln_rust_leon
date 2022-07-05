/// @author: Leon
/// https://leetcode.com/problems/fair-distribution-of-cookies/
/// Time Complexity:    O(`k` ^ `len_cs`)
/// Space Complexity:   O(`k`)
/// Reference:
/// https://leetcode.com/problems/fair-distribution-of-cookies/discuss/2140918/Easy-C%2B%2B-or-Backtracking
/// https://leetcode.com/problems/fair-distribution-of-cookies/discuss/2140918/Easy-C++-or-Backtracking/1434765
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE: i32 = (1e5 as i32 + 7) * 8;
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        let _len_cs: usize = cookies.len();
        let k: usize = k as usize;
        let mut dist: Vec<i32> = vec![0; k];
        let mut ans: i32 = Self::RANGE;
        Self::backtrack(0, &mut dist, &cookies, k, &mut ans);
        ans
    }
    fn backtrack(idx_c: usize, dist: &mut Vec<i32>, cookies: &Vec<i32>, k: usize, res: &mut i32) {
        let len_cs: usize = cookies.len();
        if idx_c == len_cs {
            let most: i32 = *dist.iter().max().unwrap();
            *res = std::cmp::min(*res, most);
            return;
        };
        for idx in 0..k {
            dist[idx] += cookies[idx_c];
            Self::backtrack(1 + idx_c, dist, cookies, k, res);
            dist[idx] -= cookies[idx_c];
            if dist[idx] == 0 {
                break;
            }
        }
    }
}
