use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/shortest-distance-from-all-buildings/
/// Time Complexity:    O((`len_rs` * `len_cs`) ^ 2)
/// Space Complexity:   O(`len_rs` * `len_cs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const BUILDING: i32 = 1;
    pub fn shortest_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let all_buildings: Vec<Building> = {
            let mut all: Vec<Building> = Vec::with_capacity(len_rs * len_cs);
            for r in 0..len_rs {
                for c in 0..len_cs {
                    if grid[r][c] == Self::BUILDING {
                        all.push(Building { r, c, distance: 0 });
                    }
                    grid[r][c] = -grid[r][c];
                }
            }
            all
        };
        let len_bs: usize = all_buildings.len();
        let mut distances: Vec<Vec<u16>> = vec![vec![0; len_cs]; len_rs];
        for (idx, building) in all_buildings.into_iter().enumerate() {
            Self::bfs(building, idx as i32, &mut distances, &mut grid);
        }
        let mut shortest: i32 = -1;
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == len_bs as i32
                    && (shortest < 0 || shortest > distances[r][c] as i32)
                {
                    shortest = distances[r][c] as i32;
                }
            }
        }
        return shortest;
    }
    fn bfs(
        building: Building,
        expected_id: i32,
        distances: &mut Vec<Vec<u16>>,
        grid: &mut Vec<Vec<i32>>,
    ) {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut queue: VecDeque<Building> = {
            let mut queue: VecDeque<Building> = VecDeque::new();
            queue.push_back(building);
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(Building { r, c, distance }) = queue.pop_front() {
                    distances[r][c] += distance;
                    for d in 0..4 {
                        let r_nxt: isize = r as isize + Self::DIRS[d];
                        let c_nxt: isize = c as isize + Self::DIRS[d + 1];
                        if r_nxt < 0 || c_nxt < 0 {
                            continue;
                        }
                        let r_nxt: usize = r_nxt as usize;
                        let c_nxt: usize = c_nxt as usize;
                        if r_nxt >= len_rs || c_nxt >= len_cs || grid[r_nxt][c_nxt] != expected_id {
                            continue;
                        }
                        grid[r_nxt][c_nxt] = 1 + expected_id;
                        queue.push_back(Building {
                            r: r_nxt,
                            c: c_nxt,
                            distance: 1 + distance,
                        });
                    }
                }
            }
        }
    }
}

struct Building {
    r: usize,
    c: usize,
    distance: u16,
}
