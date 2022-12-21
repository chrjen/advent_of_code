use regex::Regex;

pub fn blueprints(input: &str) -> Vec<super::Blueprint> {
    let reg = Regex::new(r"Blueprint (\d+):\s+Each ore robot costs (\d+) ore.\s+Each clay robot costs (\d+) ore.\s+Each obsidian robot costs (\d+) ore and (\d+) clay.\s+Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    reg.captures_iter(input)
        .map(|cap| super::Blueprint {
            ore_robot_ore: cap[2].parse().unwrap(),
            clay_robot_ore: cap[3].parse().unwrap(),
            obsidian_robot_ore: cap[4].parse().unwrap(),
            obsidian_robot_clay: cap[5].parse().unwrap(),
            geode_robot_ore: cap[6].parse().unwrap(),
            geode_robot_obsidian: cap[7].parse().unwrap(),
        })
        .collect()
}
