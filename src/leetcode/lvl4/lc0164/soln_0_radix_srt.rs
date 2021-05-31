/// https://leetcode.com/problems/maximum-gap/
///
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/maximum-gap/discuss/521415/Rust-Bucket-sort
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        // not used
        // let len_n = nums.len();
        Self::radix_sort_base10(&mut nums);
        nums.windows(2).map(|w| w[1] - w[0]).max().unwrap_or(0)
    }

    fn radix_sort_base10(nums: &mut [i32]) {
        let mut buckets = vec![vec![]; 10];
        for i in 0..10 {
            nums.iter()
                .for_each(|&x| buckets[((x / 10_i32.pow(i)) % 10) as usize].push(x));

            buckets
                .iter()
                .flat_map(|b| b.iter())
                .zip(nums.iter_mut())
                .for_each(|(&x, y)| *y = x);
            buckets.iter_mut().for_each(|b| b.clear());
        }
    }
}
