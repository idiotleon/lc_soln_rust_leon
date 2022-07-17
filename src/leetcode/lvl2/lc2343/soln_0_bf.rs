/// @author: Leon
/// Time Complexity:    O(`len_qs` * `len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let len_qs: usize = queries.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_qs);
        for query in queries {
            let k: usize = query[0] as usize;
            let trim: usize = query[1] as usize;
            let res = Self::get_kth_num(&nums, trim, k);
            ans.push(res);
        }
        ans
    }
    fn get_kth_num(nums: &Vec<String>, trim: usize, k: usize) -> i32 {
        let len_ns: usize = nums.len();
        let mut res: Vec<(&str, usize)> = Vec::with_capacity(len_ns);
        for (idx, num) in nums.iter().enumerate() {
            let len_n: usize = num.len();
            res.push((&num[len_n - trim..], idx));
        }
        res.sort_by_key(|&r| r.0);
        res[k - 1].1 as i32
    }
}
