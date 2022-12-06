use regex::Regex;

pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 21: RPG Simulator 20XX",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use Armour::*;
use Ring::*;
use Weapon::*;

const ALL_WEAPONS: &[Weapon] = &[Dagger, Shortsword, Warhammer, Longsword, Greataxe];

const ALL_ARMOUR: &[Option<Armour>] = &[
    None,
    Some(Leather),
    Some(Chainmail),
    Some(Splintmail),
    Some(Bandedmail),
    Some(Platemail),
];

const ALL_RINGS: &[[Option<Ring>; 2]] = &[
    // No rings.
    [None, None],
    // One ring.
    [Some(Damage1), None],
    [Some(Damage2), None],
    [Some(Damage3), None],
    [Some(Defence1), None],
    [Some(Defence2), None],
    [Some(Defence3), None],
    // Two rings.
    [Some(Damage1), Some(Damage2)],
    [Some(Damage1), Some(Damage3)],
    [Some(Damage1), Some(Defence1)],
    [Some(Damage1), Some(Defence2)],
    [Some(Damage1), Some(Defence3)],
    [Some(Damage2), Some(Damage3)],
    [Some(Damage2), Some(Defence1)],
    [Some(Damage2), Some(Defence2)],
    [Some(Damage2), Some(Defence3)],
    [Some(Damage3), Some(Defence1)],
    [Some(Damage3), Some(Defence2)],
    [Some(Damage3), Some(Defence3)],
    [Some(Defence1), Some(Defence2)],
    [Some(Defence1), Some(Defence3)],
    [Some(Defence2), Some(Defence3)],
];

trait Equipment {
    fn cost(&self) -> i32;
    fn damage(&self) -> i32;
    fn armour(&self) -> i32;
}

#[derive(Clone, Copy, Debug)]
enum Weapon {
    Dagger,
    Shortsword,
    Warhammer,
    Longsword,
    Greataxe,
}

#[derive(Clone, Copy, Debug)]
enum Armour {
    Leather,
    Chainmail,
    Splintmail,
    Bandedmail,
    Platemail,
}

#[derive(Clone, Copy, Debug)]
enum Ring {
    Damage1,
    Damage2,
    Damage3,
    Defence1,
    Defence2,
    Defence3,
}

struct Loadout {
    weapon: Weapon,
    armour: Option<Armour>,
    rings: [Option<Ring>; 2],
}

#[derive(Clone, Copy, Debug)]
struct Entity {
    health: i32,
    damage: i32,
    armour: i32,
}

impl Equipment for Weapon {
    fn cost(&self) -> i32 {
        match self {
            Weapon::Dagger => 8,
            Weapon::Shortsword => 10,
            Weapon::Warhammer => 25,
            Weapon::Longsword => 40,
            Weapon::Greataxe => 70,
        }
    }

    fn damage(&self) -> i32 {
        match self {
            Weapon::Dagger => 4,
            Weapon::Shortsword => 5,
            Weapon::Warhammer => 6,
            Weapon::Longsword => 7,
            Weapon::Greataxe => 8,
        }
    }

    fn armour(&self) -> i32 {
        0
    }
}

impl Equipment for Armour {
    fn cost(&self) -> i32 {
        match self {
            Armour::Leather => 13,
            Armour::Chainmail => 31,
            Armour::Splintmail => 53,
            Armour::Bandedmail => 75,
            Armour::Platemail => 102,
        }
    }

    fn damage(&self) -> i32 {
        0
    }

    fn armour(&self) -> i32 {
        match self {
            Armour::Leather => 1,
            Armour::Chainmail => 2,
            Armour::Splintmail => 3,
            Armour::Bandedmail => 4,
            Armour::Platemail => 5,
        }
    }
}

impl Equipment for Ring {
    fn cost(&self) -> i32 {
        match self {
            Ring::Damage1 => 25,
            Ring::Damage2 => 50,
            Ring::Damage3 => 100,
            Ring::Defence1 => 20,
            Ring::Defence2 => 40,
            Ring::Defence3 => 80,
        }
    }

    fn damage(&self) -> i32 {
        match self {
            Ring::Damage1 => 1,
            Ring::Damage2 => 2,
            Ring::Damage3 => 3,
            Ring::Defence1 | Ring::Defence2 | Ring::Defence3 => 0,
        }
    }

    fn armour(&self) -> i32 {
        match self {
            Ring::Damage1 | Ring::Damage2 | Ring::Damage3 => 0,
            Ring::Defence1 => 1,
            Ring::Defence2 => 2,
            Ring::Defence3 => 3,
        }
    }
}

impl Equipment for Loadout {
    fn cost(&self) -> i32 {
        self.weapon.cost()
            + self.armour.as_ref().map_or(0, |a| a.cost())
            + self.rings.iter().flatten().map(|r| r.cost()).sum::<i32>()
    }

    fn damage(&self) -> i32 {
        self.weapon.damage()
            + self.armour.as_ref().map_or(0, |a| a.damage())
            + self.rings.iter().flatten().map(|r| r.damage()).sum::<i32>()
    }

    fn armour(&self) -> i32 {
        self.weapon.armour()
            + self.armour.as_ref().map_or(0, |a| a.armour())
            + self.rings.iter().flatten().map(|r| r.armour()).sum::<i32>()
    }
}

fn div_ceil(lhs: i32, rhs: i32) -> i32 {
    (lhs + rhs - 1) / rhs
}

fn player_wins(mut player: Entity, mut boss: Entity) -> bool {
    player.damage = (player.damage - boss.armour).max(1);
    boss.damage = (boss.damage - player.armour).max(1);

    div_ceil(boss.health, player.damage) <= div_ceil(player.health, boss.damage)
}

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);

    let mut boss = Entity {
        health: 0,
        damage: 0,
        armour: 0,
    };

    let reg_attr = Regex::new(r"([A-Za-z ]+):\s+(-?\d+)").unwrap();
    for cap in reg_attr.captures_iter(&input) {
        let value: i32 = cap[2].parse().unwrap();
        match &cap[1] {
            "Hit Points" => boss.health = value,
            "Damage" => boss.damage = value,
            "Armor" => boss.armour = value,
            _ => {}
        }
    }

    let (part1, part2) = {
        let mut least_gold = i32::MAX;
        let mut most_gold = i32::MIN;
        let mut loadout = Loadout {
            weapon: Weapon::Dagger,
            armour: None,
            rings: [None, None],
        };

        let mut player = Entity {
            health: 100,
            damage: 0,
            armour: 0,
        };

        // Goes through all possible layouts and checks if the player wins or not.
        for rings in ALL_RINGS {
            loadout.rings = *rings;

            for armour in ALL_ARMOUR {
                loadout.armour = *armour;

                for weapon in ALL_WEAPONS {
                    loadout.weapon = *weapon;

                    let cost = loadout.cost();

                    player.damage = loadout.damage();
                    player.armour = loadout.armour();
                    if player_wins(player, boss) {
                        least_gold = least_gold.min(cost);
                    } else {
                        most_gold = most_gold.max(cost);
                    }
                }
            }
        }

        (least_gold, most_gold)
    };

    (part1.to_string(), part2.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::solution;

    // Part 1
    #[test]
    fn p1_example_player_wins() {
        let player = Entity {
            health: 8,
            damage: 5,
            armour: 5,
        };
        let boss = Entity {
            health: 12,
            damage: 7,
            armour: 2,
        };
        assert!(player_wins(player, boss))
    }

    #[test]
    fn p1_example_player_loses() {
        let player = Entity {
            health: 8,
            damage: 5,
            armour: 5,
        };
        let boss = Entity {
            health: 12,
            damage: 7,
            armour: 3,
        };
        assert!(!player_wins(player, boss))
    }
    solution!(p1, p1_solution, "78");

    // Part 2
    solution!(p2, p2_solution, "148");
}
