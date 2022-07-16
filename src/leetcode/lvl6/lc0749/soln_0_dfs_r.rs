use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/contain-virus/
/// Time Complexity:    O(`len_rs` * `len_cs` * lg(`len_rs` * `len_cs`))
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/contain-virus/discuss/526848/Java-DFS-9-ms-Explanation-with-comments/803930
/// https://leetcode.com/problems/contain-virus/discuss/526848/Java-DFS-9-ms-Explanation-with-comments
/// https://leetcode.com/problems/contain-virus/discuss/2169784/Infinite-Grid-DSU-and-DFS-Modular-w-Diagram
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const INFECTED: i32 = 1;
    const UNINFECTED: i32 = 0;
    const BLOCKED: i32 = -1;
    pub fn contain_virus(mut grid: Vec<Vec<i32>>) -> i32 {
        let _len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut cnt: i32 = 0;
        let mut phase: i32 = 1;
        let mut heap: BinaryHeap<Region> = {
            let mut heap: BinaryHeap<Region> = BinaryHeap::new();
            Self::add(phase, &mut grid, &mut heap);
            heap
        };
        while let Some(Region {
            infected,
            uninfected_neighbors: _,
            walls_needed,
        }) = heap.pop()
        {
            cnt += walls_needed;
            for hash in infected {
                grid[hash / len_cs][hash % len_cs] = Self::BLOCKED;
            }
            phase += 1;
            while let Some(Region {
                infected: _,
                uninfected_neighbors,
                walls_needed: _,
            }) = heap.pop()
            {
                for hash in uninfected_neighbors {
                    grid[hash / len_cs][hash % len_cs] = phase;
                }
            }
            Self::add(phase, &mut grid, &mut heap);
        }
        cnt
    }
    fn add(phase: i32, grid: &mut Vec<Vec<i32>>, heap: &mut BinaryHeap<Region>) {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == phase {
                    let mut region = Region::new();
                    Self::dfs((r as isize, c as isize), phase, grid, &mut region);
                    if !region.uninfected_neighbors.is_empty() {
                        heap.push(region);
                    }
                }
            }
        }
    }
    fn dfs(coord: (isize, isize), phase: i32, grid: &mut Vec<Vec<i32>>, region: &mut Region) {
        let len_rs: isize = grid.len() as isize;
        let len_cs: isize = grid[0].len() as isize;
        let (r, c) = coord;
        if r < 0
            || c < 0
            || r >= len_rs
            || c >= len_cs
            || grid[r as usize][c as usize] == -1
            || grid[r as usize][c as usize] > phase
        {
            return;
        }
        if grid[r as usize][c as usize] == Self::UNINFECTED {
            region.uninfected_neighbors.insert(Self::hash(r, c, len_cs));
            region.walls_needed += 1;
            return;
        }
        grid[r as usize][c as usize] += 1;
        region.infected.insert(Self::hash(r, c, len_cs));
        for d in 0..4 {
            let r_nxt: isize = r + Self::DIRS[d];
            let c_nxt: isize = c + Self::DIRS[d + 1];
            Self::dfs((r_nxt, c_nxt), phase, grid, region);
        }
    }
    fn hash(r: isize, c: isize, len_cs: isize) -> usize {
        (r * len_cs + c) as usize
    }
}

struct Region {
    infected: HashSet<usize>,
    uninfected_neighbors: HashSet<usize>,
    walls_needed: i32,
}

impl Region {
    pub fn new() -> Self {
        Region {
            infected: HashSet::new(),
            uninfected_neighbors: HashSet::new(),
            walls_needed: 0,
        }
    }
}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uninfected_neighbors
            .len()
            .cmp(&other.uninfected_neighbors.len())
    }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.uninfected_neighbors.len() == other.uninfected_neighbors.len()
    }
}

impl Eq for Region {}
