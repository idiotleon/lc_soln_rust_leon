/// @author: Leon
/// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs-ii/
/// Time Complexity:    O(`_len_ws` * lg(`_len_ws`))
/// Space Complexity:   O(`_len_ws`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_time(jobs: Vec<i32>, workers: Vec<i32>) -> i32 {
        let _len_ws: usize = workers.len();
        let jobs = {
            let mut jobs = jobs;
            jobs.sort();
            jobs
        };
        let workers = {
            let mut workers = workers;
            workers.sort();
            workers
        };
        let mut largest: i32 = 0;
        for (idx, job) in jobs.into_iter().enumerate() {
            let worker = workers[idx];
            largest = std::cmp::max(largest, (job + worker - 1) / worker);
        }
        largest
    }
}
