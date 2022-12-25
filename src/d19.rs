use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

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
        .take(3)
        .collect_vec();
    let sum: usize = bps
        .par_iter()
        .enumerate()
        .map(|(idx, bp)| {
            let s = score(idx, bp);
            println!("{idx}: {s}");
            s
        })
        .product();
    println!("{sum}")
}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
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

fn score(hint: usize, costs: &Cost) -> usize {
    let end = 32;

    let mut init = State::default();
    init.robot_ore = 1;
    let mut previous_states = vec![init];
    for minute in 1.. {
        let mut states = HashSet::with_capacity(previous_states.len() * 5);

        for state in previous_states {
            if state.ore >= costs.geo_robot_ore && state.obs >= costs.geo_robot_obs {
                let mut state = state.step();
                state.ore -= costs.geo_robot_ore;
                state.obs -= costs.geo_robot_obs;
                state.robot_geo += 1;
                states.insert(state);
                continue;
            }

            if state.ore >= costs.obs_robot_ore && state.clay >= costs.obs_robot_clay {
                let mut state = state.step();
                state.ore -= costs.obs_robot_ore;
                state.clay -= costs.obs_robot_clay;
                state.robot_obs += 1;
                states.insert(state);
            }

            if state.ore >= costs.clay_robot_ore {
                let mut state = state.step();
                state.ore -= costs.clay_robot_ore;
                state.robot_clay += 1;
                states.insert(state);
            }

            if state.ore >= costs.ore_robot_ore {
                let mut state = state.step();
                state.ore -= costs.ore_robot_ore;
                state.robot_ore += 1;
                states.insert(state);
            }

            states.insert(state.step());
        }

        let rem = end - minute;
        let no_action_expected = |state: &State| state.geo + rem * state.robot_geo;
        let expected = states
            .iter()
            .map(no_action_expected)
            .max()
            .unwrap_or_default();

        println!(
            "{hint} minute {minute}, {} states, expected: {expected:?}",
            states.len()
        );

        if minute == end {
            return states.iter().map(|s| s.geo).max().unwrap();
        }

        let possible_to_make = (rem) * (rem + 1) / 2;

        previous_states = states
            .into_iter()
            .filter(|s| no_action_expected(s) + possible_to_make >= expected)
            .collect();
    }

    unreachable!();
}
