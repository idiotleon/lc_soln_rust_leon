/// https://leetcode.com/problems/design-underground-system/
///
/// Time Complexities:
///
/// Space Complexity:   O()
use std::collections::HashMap;

#[derive(Default)]
#[allow(dead_code)]
struct UndergroundSystem {
    check_out_map: HashMap<(String, String), (i32, usize)>,
    check_in_map: HashMap<i32, (String, i32)>,
}

#[allow(dead_code)]
impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.check_in_map.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, check_out_station: String, t: i32) {
        if let Some((start_station, start_time)) = self.check_in_map.get(&id) {
            let check_out = self
                .check_out_map
                .entry((start_station.clone(), check_out_station))
                .or_default();

            check_out.0 += t - start_time;
            check_out.1 += 1;
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(&(total_stations, total_time)) =
            self.check_out_map.get(&(start_station, end_station))
        {
            return f64::from(total_stations) / total_time as f64;
        }

        unreachable!()
    }
}
