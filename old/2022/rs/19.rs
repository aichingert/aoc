const PART_ONE: u8 = 24;
const PART_TWO: u8 = 32;

struct Blueprint {
    id: u8,
    max_ore: u8,
    ore_r: u8,
    cly_r: u8,
    obs_r: (u8, u8),
    geo_r: (u8, u8),
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Pack {
    ore_r: u8,
    cly_r: u8,
    obs_r: u8,
    geo_r: u8,
    ore: u8,
    cly: u8,
    obs: u8,
    geo: u8,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct State {
    minute: u8,
    pack: Pack,
}

impl State {
    pub fn new() -> Self {
        Self {
            minute: 0,
            pack: Pack {
                ore_r: 1,
                cly_r: 0,
                obs_r: 0,
                geo_r: 0,
                ore: 0,
                cly: 0,
                obs: 0,
                geo: 0,
            },
        }
    }

    pub fn can_build_ore_robot_and_is_necessary(&self, bp: &Blueprint) -> bool {
        self.pack.ore >= bp.ore_r && bp.max_ore > self.pack.ore_r
    }

    pub fn can_build_cly_robot_and_is_necessary(&self, bp: &Blueprint) -> bool {
        self.pack.ore >= bp.cly_r && bp.obs_r.1 > self.pack.cly_r
    }
    pub fn can_build_obs_robot_and_is_necessary(&self, bp: &Blueprint) -> bool {
        self.pack.ore >= bp.obs_r.0 && self.pack.cly >= bp.obs_r.1 && bp.geo_r.1 > self.pack.obs_r
    }
    pub fn can_build_geo_robot(&self, bp: &Blueprint) -> bool {
        self.pack.ore >= bp.geo_r.0 && self.pack.obs >= bp.geo_r.1
    }

    pub fn build_ore_robot(mut self, bp: &Blueprint) -> Self {
        self.pack.ore -= bp.ore_r;
        self.pack.ore_r += 1;
        self
    }

    pub fn build_cly_robot(mut self, bp: &Blueprint) -> Self {
        self.pack.ore -= bp.cly_r;
        self.pack.cly_r += 1;
        self
    }

    pub fn build_obs_robot(mut self, bp: &Blueprint) -> Self {
        self.pack.ore -= bp.obs_r.0;
        self.pack.cly -= bp.obs_r.1;
        self.pack.obs_r += 1;
        self
    }

    pub fn build_geo_robot(mut self, bp: &Blueprint) -> Self {
        self.pack.ore -= bp.geo_r.0;
        self.pack.obs -= bp.geo_r.1;
        self.pack.geo_r += 1;
        self
    }

    pub fn step(mut self) -> Self {
        self.minute += 1;
        self.pack.ore += self.pack.ore_r;
        self.pack.cly += self.pack.cly_r;
        self.pack.obs += self.pack.obs_r;
        self.pack.geo += self.pack.geo_r;
        self
    }

    pub fn cant_beat(&self, limit: u8, max_result: u8) -> bool {
        let remaining = (limit - self.minute) as u32;
        let max_geodes = remaining * (self.pack.geo_r as u32) + (remaining * (remaining - 1)) / 2;
        self.pack.geo as u32 + max_geodes <= max_result as u32
    }
}

impl Blueprint {
    fn new(s: &str) -> Self {
        let (bp, values) = s.split_once(": ").unwrap();
        let costs = values.split(' ').collect::<Vec<_>>();
        let ore_r: u8 = costs[4].parse().unwrap();
        let cly_r: u8 = costs[10].parse().unwrap();
        let obs_r: (u8,u8) = (costs[16].parse().unwrap(), costs[19].parse().unwrap());
        let geo_r: (u8,u8) = (costs[25].parse().unwrap(), costs[28].parse().unwrap());

        let mut max_ore = 0;
        if ore_r > max_ore { max_ore = ore_r; }
        if cly_r > max_ore { max_ore = cly_r; }
        if obs_r.0 > max_ore { max_ore = obs_r.0; }
        if geo_r.0 > max_ore { max_ore = geo_r.0; }

        Self {
            id: bp.split(' ').collect::<Vec<_>>()[1].parse().unwrap(),
            max_ore,
            ore_r,
            cly_r,
            obs_r,
            geo_r,
        }
    }

    fn solve(&self, limit: u8, state: State, max_result: &mut u8) -> u8 {
        if state.cant_beat(limit, *max_result) {
            return 0;
        }

        if state.minute >= limit {
            if *max_result < state.pack.geo { *max_result = state.pack.geo; }
            return state.pack.geo;
        }

        let mut result = 0;

        if state.can_build_geo_robot(self) {
            result = result.max(self.solve(limit, state.step().build_geo_robot(self), max_result));
        }

        if state.can_build_obs_robot_and_is_necessary(self) {
            result = result.max(self.solve(limit, state.step().build_obs_robot(self), max_result));
        }

        if state.can_build_cly_robot_and_is_necessary(self) {
            result = result.max(self.solve(limit, state.step().build_cly_robot(self), max_result));
        }

        if state.can_build_ore_robot_and_is_necessary(self) {
            result = result.max(self.solve(limit, state.step().build_ore_robot(self), max_result));
        }

        result.max(self.solve(limit, state.step(), max_result))
    }
}

fn part_one(bps: &Vec<Blueprint>) -> u32 {
    bps.iter().map(|bp| bp.id as u32 * bp.solve(PART_ONE, State::new(), &mut 0) as u32).sum::<u32>()
}

fn part_two(bps: &Vec<Blueprint>) -> u32 {
    bps.iter().take(3).map(|bp| bp.solve(PART_TWO, State::new(), &mut 0) as u32).fold(1, |acc, cur| acc * cur)
}

fn main() {
    let inp = std::fs::read_to_string("../input/19").unwrap().trim().to_string();
    let bps: Vec<Blueprint> = inp.lines().map(|l| Blueprint::new(l)).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&bps));
    println!("Part one: {}", part_two(&bps));
}
