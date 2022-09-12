use aoc::read_to_slice;
use std::collections::HashMap;

pub struct Aoc2018_04 {
    g: Vec<Guard>
}

#[derive(Debug)]
struct Guard {
    id: i32,
    times: Vec<(i32, i32)>
}
        
impl Aoc2018_04 {
    pub fn new() -> Self {
        Self { g: vec![] }
    }
}

impl Guard {
    fn new(id: i32, times: Vec<(i32, i32)>) -> Self {
        Self {
            id,
            times
        }
    }
}
        
impl crate::Solution for Aoc2018_04 {
    fn name(&self) -> (usize, usize) {
        (2018, 04)
    }
        
    fn parse(&mut self) {
        let mut d = read_to_slice("input/2018/04.txt", " ");
        let mut s: Vec<Vec<String>> = vec![];

        while d.len() > 0 {
            let mut position: usize = 0;
            let mut date: (i32, i32, i32, i32, i32) = (10000, 1000, 100, 60, 60);
            for i in 0..d.len() {
                let current_date: (i32, i32, i32, i32, i32) = (d[i][0][1..=4].parse().expect("unable to parse year"), 
                d[i][0][6..=7].parse().expect("unable to parse month"),
                d[i][0][9..=10].parse().expect("unable to parse day"), 
                d[i][1][0..=1].parse().expect("unable to parse hour"),
                d[i][1][3..=4].parse().expect("unable to parse minute"));

                if current_date.0 < date.0 {
                    date = current_date;
                    position = i;
                } else if current_date.0 <= date.0 && current_date.1 < date.1 {
                    date = current_date;
                    position = i;
                } else if current_date.0 <= date.0 && current_date.1 <= date.1 && current_date.2 < date.2 {
                    date = current_date;
                    position = i;
                } else if current_date.0 <= date.0 && current_date.1 <= date.1 && current_date.2 <= date.2 && current_date.3 < date.3 {
                    date = current_date;
                    position = i;
                } else if current_date.0 <= date.0 && current_date.1 <= date.1 && current_date.2 <= date.2 && current_date.3 <= date.3 && current_date.4 < date.4 {
                    date = current_date;
                    position = i;
                }
            }

            s.push(d.remove(position));
        }

        let mut current_id: i32 = 0;
        let mut times: Vec<(i32, i32)> = vec![];
        let mut new_time: (i32, i32) = (0, 0);
        let mut awake: bool = false;

        for i in 0..s.len() {
            if &s[i][2] == "Guard" {
                let mut found: bool = false;

                for j in 0..self.g.len() {
                    if self.g[j].id == current_id {
                        for time in times.iter() {
                            self.g[j].times.push(*time);
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                    self.g.push(Guard::new(current_id, times.clone()))
                }
                times.clear();
                awake = false;
                current_id = s[i][3][1..s[i][3].len()].parse().expect("unable to parse id");
            } else {
                if !awake {
                    new_time.0 = s[i][1][s[i][1].len() - 3..s[i][1].len() - 1].parse().expect("unable to parse time");
                    awake = true;
                } else {
                    new_time.1 = s[i][1][s[i][1].len() - 3..s[i][1].len() - 1].parse().expect("unable to parse time");
                    awake = false;
                    times.push(new_time);
                }
            }
        }

        for i in 0..self.g.len() {
            if self.g[i].id == current_id {
                for time in times.iter() {
                    self.g[i].times.push(*time);
                }
                break;
            }
        }

        self.g.remove(0);
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut most_asleep: (i32, i32) = (0, 0);
        let mut asleep_mins: HashMap<i32, i32> = HashMap::new();

        for i in 0..self.g.len() {
            let mut sum: i32 = 0;
            for j in 0..self.g[i].times.len() {
                sum += self.g[i].times[j].1 - self.g[i].times[j].0;
            }

            if sum > most_asleep.1 {
                most_asleep = (self.g[i].id, sum);
            }
        }

        for i in 0..self.g.len() {
            if self.g[i].id == most_asleep.0 {
                for j in 0..self.g[i].times.len() {
                    for k in self.g[i].times[j].0..self.g[i].times[j].1 {
                        let minute: Option<i32> = asleep_mins.remove(&k);

                        if let Some(min) = minute {
                            asleep_mins.insert(k, min + 1);
                        } else {
                            asleep_mins.insert(k, 1);
                        }
                    }
                }

                break;
            }
        }

        let mut most_mins: (i32, i32) = (0, 0);

        for k in asleep_mins.keys() {
            if asleep_mins[k] > most_mins.1 {
                most_mins = (*k, asleep_mins[k]);
            }
        }

        crate::output(most_asleep.0 * most_mins.0)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut result: (i32, i32, i32) = (0, 0, 0);

        for i in 0..self.g.len() {
            let mut asleep_mins: HashMap<i32, i32> = HashMap::new();

            for j in 0..self.g[i].times.len() {
                for k in self.g[i].times[j].0..self.g[i].times[j].1 {
                    if let Some(min) = asleep_mins.remove(&k) {
                        asleep_mins.insert(k, min + 1);
                    } else {
                        asleep_mins.insert(k, 1);
                    }
                }
            }

            let mut most_mins: (i32, i32) = (0, 0);

            for k in asleep_mins.keys() {
                if asleep_mins[k] > most_mins.1 {
                    most_mins = (*k, asleep_mins[k]);
                }
            }

            if result.2 < most_mins.1 {
                result = (self.g[i].id, most_mins.0, most_mins.1);
            }
        }

        crate::output(result.0 * result.1)
    }
}