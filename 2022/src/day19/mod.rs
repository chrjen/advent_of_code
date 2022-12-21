use std::sync::{atomic::AtomicU32, Arc};

mod parse;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 19: Not Enough Minerals",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

#[derive(Debug, Clone)]
struct State {
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
    ore_resource: u32,
    clay_resource: u32,
    obsidian_resource: u32,
    geode_resource: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
            ore_resource: 0,
            clay_resource: 0,
            obsidian_resource: 0,
            geode_resource: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub ore_robot_ore: u32,
    pub clay_robot_ore: u32,
    pub obsidian_robot_ore: u32,
    pub obsidian_robot_clay: u32,
    pub geode_robot_ore: u32,
    pub geode_robot_obsidian: u32,
}

/// Does division, but rounds the answer up in case of a fractional answer.
fn div_ceil(lhs: u32, rhs: u32) -> u32 {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if r > 0 {
        d + 1
    } else {
        d
    }
}

/// Recursively goes through all possible states searching for the best answer.
/// Some branch pruning are implemented to avoid wasting time exploring states
/// that cannot possibly be any better than the current state.  
fn max_geodes(time: u32, state: State, blueprint: &Blueprint) -> u32 {
    // As all robots requires ore, we calculate the blueprint that has the highest
    // ore requirement here to avoid doing it multiple times.
    let max_ore = blueprint
        .ore_robot_ore
        .max(blueprint.clay_robot_ore)
        .max(blueprint.obsidian_robot_ore)
        .max(blueprint.geode_robot_ore);

    fn max_geodes_(time: u32, state: State, blueprint: &Blueprint, max_ore: u32) -> u32 {
        // With 1 or fewer minutes left we do nothing as building a robot would
        // not have enough time to make any resources before running out of time.
        if time <= 1 {
            return state.geode_resource + time * state.geode_robot;
        }

        let collect_resources = |time_left| {
            let mut new_state = state.clone();
            new_state.ore_resource += time_left * new_state.ore_robot;
            new_state.clay_resource += time_left * new_state.clay_robot;
            new_state.obsidian_resource += time_left * new_state.obsidian_robot;
            new_state.geode_resource += time_left * new_state.geode_robot;
            new_state
        };

        let mut geodes = 0;

        // Build an Ore Robot.
        if time * state.ore_robot + state.ore_resource < time * max_ore {
            let ores_left = blueprint.ore_robot_ore.saturating_sub(state.ore_resource);
            let time_left = div_ceil(ores_left, state.ore_robot) + 1;
            if time_left <= time {
                let mut new_state = collect_resources(time_left);
                new_state.ore_resource -= blueprint.ore_robot_ore;
                new_state.ore_robot += 1;
                geodes = geodes.max(max_geodes_(time - time_left, new_state, blueprint, max_ore));
            }
        }

        // Build an Clay Robot.
        if time * state.clay_robot + state.clay_resource < time * blueprint.obsidian_robot_clay {
            let ores_left = blueprint.clay_robot_ore.saturating_sub(state.ore_resource);
            let time_left = div_ceil(ores_left, state.ore_robot) + 1;
            if time_left <= time {
                let mut new_state = collect_resources(time_left);
                new_state.ore_resource -= blueprint.clay_robot_ore;
                new_state.clay_robot += 1;
                geodes = geodes.max(max_geodes_(time - time_left, new_state, blueprint, max_ore));
            }
        }

        // Build an Obsidian Robot.
        if state.clay_robot != 0
            && time * state.obsidian_robot + state.obsidian_resource
                < time * blueprint.geode_robot_obsidian
        {
            let ores_left = blueprint
                .obsidian_robot_ore
                .saturating_sub(state.ore_resource);
            let clay_left = blueprint
                .obsidian_robot_clay
                .saturating_sub(state.clay_resource);
            let time_left = (div_ceil(ores_left, state.ore_robot) + 1)
                .max(div_ceil(clay_left, state.clay_robot) + 1);
            if time_left <= time {
                let mut new_state = collect_resources(time_left);
                new_state.ore_resource -= blueprint.obsidian_robot_ore;
                new_state.clay_resource -= blueprint.obsidian_robot_clay;
                new_state.obsidian_robot += 1;
                geodes = geodes.max(max_geodes_(time - time_left, new_state, blueprint, max_ore));
            }
        }

        // Build an Geode Robot.
        if state.obsidian_robot != 0 {
            let ores_left = blueprint.geode_robot_ore.saturating_sub(state.ore_resource);
            let obsidian_left = blueprint
                .geode_robot_obsidian
                .saturating_sub(state.obsidian_resource);
            let time_left = (div_ceil(ores_left, state.ore_robot) + 1)
                .max(div_ceil(obsidian_left, state.obsidian_robot) + 1);
            if time_left <= time {
                let mut new_state = collect_resources(time_left);
                new_state.ore_resource -= blueprint.geode_robot_ore;
                new_state.obsidian_resource -= blueprint.geode_robot_obsidian;
                new_state.geode_robot += 1;
                geodes = geodes.max(max_geodes_(time - time_left, new_state, blueprint, max_ore));
            }
        }

        // Do nothing.
        geodes = geodes.max(state.geode_resource + time * state.geode_robot);

        geodes
    }

    max_geodes_(time, state, blueprint, max_ore)
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let blueprints = parse::blueprints(input.as_ref());

    // Part 1.
    let geodes = Arc::new(AtomicU32::new(0));
    let mut handles = Vec::new();

    for (i, blueprint) in blueprints.iter().enumerate() {
        let blueprint = blueprint.clone();
        let geodes = geodes.clone();
        let handle = std::thread::spawn(move || {
            let quality = (i as u32 + 1) * max_geodes(24, State::default(), &blueprint);
            geodes.fetch_add(quality, std::sync::atomic::Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    let part1 = geodes.load(std::sync::atomic::Ordering::SeqCst);

    // Part 2.
    let geodes = Arc::new(AtomicU32::new(1));
    let mut handles = Vec::new();

    for blueprint in blueprints.iter().take(3) {
        let blueprint = blueprint.clone();
        let geodes = geodes.clone();
        let handle = std::thread::spawn(move || {
            let max = max_geodes(32, State::default(), &blueprint);
            geodes
                .fetch_update(
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                    |v| Some(v * max),
                )
                .unwrap();
        });
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
    let part2 = geodes.load(std::sync::atomic::Ordering::Relaxed);

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use common::{example, solution};

    // Part 1
    example!(
        p1,
        p1_example_1,
        "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.",
        "33"
    );
    solution!(p1, p1_solution, "2160");

    // Part 2
    example!(
        p2,
        p2_example_1,
        "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.
",
        "56"
    );
    example!(
        p2,
        p2_example_2,
        "Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.",
        "62"
    );
    solution!(p2, p2_solution, "13340");
}
