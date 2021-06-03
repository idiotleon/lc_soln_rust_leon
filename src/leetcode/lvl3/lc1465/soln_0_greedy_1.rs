/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/discuss/1248666/Rust-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut hc = horizontal_cuts;
        let mut vc = vertical_cuts;
        hc.extend(&[0, h]);
        vc.extend(&[0, w]);
        hc.sort_unstable();
        vc.sort_unstable();
        let h_max_gap = hc.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64;
        let v_max_gap = vc.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64;
        (h_max_gap * v_max_gap % MOD) as i32
    }
}
