/// @author: Leon
/// https://leetcode.com/problems/the-earliest-moment-when-everyone-become-friends/
/// Time Complexity:    O(`_len_lgs` * lg(`_len_lgs`))
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
        let _len_lgs: usize = logs.len();
        let sorted = {
            let mut sorted = logs;
            sorted.sort();
            sorted
        };
        let mut uf = UnionFind::new(n as usize);
        for log in sorted {
            let timestamp = log[0];
            let x = log[1] as usize;
            let y = log[2] as usize;
            uf.union(x, y);
            if uf.cnt_iso == 1 {
                return timestamp;
            }
        }
        -1
    }
}

struct UnionFind {
    cnt_iso: u8,
    roots: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let roots: Vec<usize> = {
            let mut roots = vec![0; n];
            for idx in 0..n {
                roots[idx] = idx;
            }
            roots
        };
        UnionFind {
            cnt_iso: n as u8,
            roots,
            ranks: vec![1; n],
        }
    }
    fn union(&mut self, x: usize, y: usize) {
        let root_x: usize = self.find(x);
        let root_y: usize = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.ranks[root_x] > self.ranks[root_y] {
            self.roots[root_y] = root_x;
            self.ranks[root_x] += 1;
        } else {
            self.roots[root_x] = root_y;
            self.ranks[root_y] += 1;
        }
        self.cnt_iso -= 1;
    }
    fn find(&mut self, x: usize) -> usize {
        if self.roots[x] != x {
            self.roots[x] = self.find(self.roots[x]);
        }
        self.roots[x]
    }
}
