/// https://leetcode.com/problems/longest-duplicate-substring/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/longest-duplicate-substring/discuss/695243/Rust-binary-search-%2B-Rabin-Karp-solution
use std::collections::HashMap;

const M: u64 = (1 << 31) - 1;
const B: u64 = 257;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let s: &[u8] = s.as_bytes();
        let mut hashes: Vec<u64> = Vec::with_capacity(s.len() + 1);
        let mut hash = 0;
        hashes.push(hash);
        for &u in s.iter() {
            hash = (hash * B + u as u64) % M;
            hashes.push(hash);
        }
        let (mut lo, mut hi) = (0, s.len());
        let mut ans: usize = 0;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if let Some(a) = Self::helper(s, mid, &hashes) {
                ans = a;
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        s[ans..ans + lo - 1].iter().map(|&u| u as char).collect()
    }
    fn helper(s: &[u8], len: usize, hashes: &[u64]) -> Option<usize> {
        let b = (0..len as u64).fold(1, |acc, _| (acc * B) % M);
        let mut hm: HashMap<u64, usize> = HashMap::new();
        for i in 0..s.len() - len + 1 {
            let h = (hashes[i + len] + M - hashes[i] * b % M) % M;
            if let Some(j) = hm.get(&h) {
                if (0..len).all(|k| s[i + k] == s[j + k]) {
                    return Some(*j);
                }
            } else {
                hm.insert(h, i);
            }
        }
        None
    }
}
