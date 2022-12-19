use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Cost {
    ore_robot_ore: usize,
    clay_robot_ore: usize,
    obs_robot_ore: usize,
    obs_robot_clay: usize,
    geo_robot_ore: usize,
    geo_robot_obs: usize,
}

pub fn solve() {
    let bps = include_str!("d19.txt")
        .lines()
        .map(|l| {
            let nums = l
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec();
            Cost {
                ore_robot_ore: nums[0],
                clay_robot_ore: nums[1],
                obs_robot_ore: nums[2],
                obs_robot_clay: nums[3],
                geo_robot_ore: nums[4],
                geo_robot_obs: nums[5],
            }
        })
        .collect_vec();
    let mut sum = 0;
    for (idx, bp) in bps.iter().enumerate() {
        let s = score(bp);
        println!("{idx}: {s}");
        sum += (idx + 1) * s;
    }
    println!("{sum}")
}

#[derive(Copy, Clone, Default, Debug)]
struct State {
    ore: usize,
    clay: usize,
    obs: usize,
    geo: usize,

    robot_ore: usize,
    robot_clay: usize,
    robot_obs: usize,
    robot_geo: usize,
}

impl State {
    fn step(self) -> State {
        let mut state = self.clone();
        state.ore += state.robot_ore;
        state.obs += state.robot_obs;
        state.geo += state.robot_geo;
        state.clay += state.robot_clay;
        state
    }
}

fn score(costs: &Cost) -> usize {
    let mut init = State::default();
    init.robot_ore = 1;
    search(costs, init, 1)
}

fn search(costs: &Cost, state: State, minute: u8) -> usize {
    if minute == 24 {
        let state = state.step();
        // if state.robot_ore == 1
        //     && state.robot_clay == 4
        //     && state.robot_obs == 2
        //     && state.robot_geo == 2
        //     && state.geo == 9
        // {
        //     println!("{state:?}");
        // }
        // if state.geo > 8 {
        //     println!("{state:?}");
        // }
        return state.geo;
    }

    let mut found = 0;

    if state.ore >= costs.geo_robot_ore && state.obs >= costs.geo_robot_obs {
        let mut state = state.step();
        state.ore -= costs.geo_robot_ore;
        state.obs -= costs.geo_robot_obs;
        state.robot_geo += 1;
        return search(costs, state, minute + 1);
    }

    if state.ore >= costs.obs_robot_ore && state.clay >= costs.obs_robot_clay {
        let mut state = state.step();
        state.ore -= costs.obs_robot_ore;
        state.clay -= costs.obs_robot_clay;
        state.robot_obs += 1;
        found = found.max(search(costs, state, minute + 1));
    }

    if state.ore >= costs.clay_robot_ore {
        let mut state = state.step();
        state.ore -= costs.clay_robot_ore;
        state.robot_clay += 1;
        found = found.max(search(costs, state, minute + 1));
    }

    if state.ore >= costs.ore_robot_ore {
        let mut state = state.step();
        state.ore -= costs.ore_robot_ore;
        state.robot_ore += 1;
        found = found.max(search(costs, state, minute + 1));
    }

    found = found.max(search(costs, state.step(), minute + 1));

    found
}
