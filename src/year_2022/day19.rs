use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, newline, u8},
    combinator::{map, opt},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct RobotPrices {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

#[derive(Debug)]
struct Blueprint {
    id: u8,
    ore_cost: RobotPrices,
    clay_cost: RobotPrices,
    obsidian_cost: RobotPrices,
    geode_cost: RobotPrices,
    max_ore_bots: u8,
    max_clay_bots: u8,
    max_obsidian_bots: u8,
}

fn material_cost<'a>(material: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, u8> {
    map(
        opt(delimited(
            delimited(multispace0, opt(tag("and")), multispace0),
            u8,
            pair(multispace0, tag(material)),
        )),
        |n| n.unwrap_or(0),
    )
}

fn robot_prices(input: &str) -> IResult<&str, RobotPrices> {
    let (input, _) = multispace0(input)?;
    let (input, _) = delimited(tag("Each "), alpha1, tag(" robot costs"))(input)?;
    let (input, ore) = material_cost("ore")(input)?;
    let (input, clay) = material_cost("clay")(input)?;
    let (input, obsidian) = material_cost("obsidian")(input)?;
    let (input, _) = tag(".")(input)?;

    Ok((
        input,
        RobotPrices {
            ore,
            clay,
            obsidian,
        },
    ))
}

fn blueprint(input: &str) -> IResult<&str, Blueprint> {
    let mut max_ore_bots = 0;
    let mut max_clay_bots = 0;
    let mut max_obsidian_bots = 0;

    let (input, _) = multispace0(input)?;
    let (input, id) = delimited(tag("Blueprint "), u8, tag(":"))(input)?;
    let (input, ore_cost) = robot_prices(input)?;
    max_ore_bots = max_ore_bots.max(ore_cost.ore);
    let (input, clay_cost) = robot_prices(input)?;
    max_ore_bots = max_ore_bots.max(clay_cost.ore);
    let (input, obsidian_cost) = robot_prices(input)?;
    max_ore_bots = max_ore_bots.max(obsidian_cost.ore);
    max_clay_bots = max_clay_bots.max(obsidian_cost.clay);
    let (input, geode_cost) = robot_prices(input)?;
    max_ore_bots = max_ore_bots.max(geode_cost.ore);
    max_obsidian_bots = max_obsidian_bots.max(geode_cost.obsidian);

    Ok((
        input,
        Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
            max_ore_bots,
            max_clay_bots,
            max_obsidian_bots,
        },
    ))
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Inventory {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
    ore_bots: u8,
    clay_bots: u8,
    obsidian_bots: u8,
    geode_bots: u8,
    todo_ore_bots: u8,
    todo_clay_bots: u8,
    todo_obsidian_bots: u8,
    todo_geode_bots: u8,
    minute: u8,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
            todo_ore_bots: 0,
            todo_clay_bots: 0,
            todo_obsidian_bots: 0,
            todo_geode_bots: 0,
            minute: 0,
        }
    }
}

impl Inventory {
    fn has_items(&self, price: &RobotPrices) -> bool {
        self.ore >= price.ore && self.clay >= price.clay && self.obsidian >= price.obsidian
    }

    fn deduct_items(&mut self, price: &RobotPrices) {
        self.ore -= price.ore;
        self.clay -= price.clay;
        self.obsidian -= price.obsidian;
    }

    fn try_buy_ore_bot(&self, blueprint: &Blueprint) -> Option<Inventory> {
        if self.ore_bots >= blueprint.max_ore_bots {
            None
        } else if self.has_items(&blueprint.ore_cost) {
            let mut new_inv = *self;

            new_inv.deduct_items(&blueprint.ore_cost);
            new_inv.todo_ore_bots += 1;
            Some(new_inv)
        } else {
            None
        }
    }

    fn try_buy_clay_bot(&self, blueprint: &Blueprint) -> Option<Inventory> {
        if self.ore_bots >= blueprint.max_clay_bots {
            None
        } else if self.has_items(&blueprint.clay_cost) {
            let mut new_inv = *self;

            new_inv.deduct_items(&blueprint.clay_cost);
            new_inv.todo_clay_bots += 1;
            Some(new_inv)
        } else {
            None
        }
    }

    fn try_buy_obsidian_bot(&self, blueprint: &Blueprint) -> Option<Inventory> {
        if self.ore_bots >= blueprint.max_obsidian_bots {
            None
        } else if self.has_items(&blueprint.obsidian_cost) {
            let mut new_inv = *self;

            new_inv.deduct_items(&blueprint.obsidian_cost);
            new_inv.todo_obsidian_bots += 1;
            Some(new_inv)
        } else {
            None
        }
    }

    fn try_buy_geode_bot(&self, blueprint: &Blueprint) -> Option<Inventory> {
        if self.has_items(&blueprint.geode_cost) {
            let mut new_inv = *self;

            new_inv.deduct_items(&blueprint.geode_cost);
            new_inv.todo_geode_bots += 1;
            Some(new_inv)
        } else {
            None
        }
    }

    fn advance_minute(&self) -> Inventory {
        let mut new_inv = *self;
        new_inv.ore += new_inv.ore_bots;
        new_inv.clay += new_inv.clay_bots;
        new_inv.obsidian += new_inv.obsidian_bots;
        new_inv.geodes += new_inv.geode_bots;

        new_inv.ore_bots += new_inv.todo_ore_bots;
        new_inv.clay_bots += new_inv.todo_clay_bots;
        new_inv.obsidian_bots += new_inv.todo_obsidian_bots;
        new_inv.geode_bots += new_inv.todo_geode_bots;

        new_inv.todo_ore_bots = 0;
        new_inv.todo_clay_bots = 0;
        new_inv.todo_obsidian_bots = 0;
        new_inv.todo_geode_bots = 0;

        new_inv.minute += 1;

        new_inv
    }

    fn worse_than(&self, inv: &Inventory) -> bool {
        self.ore <= inv.ore
            && self.clay <= inv.clay
            && self.obsidian <= inv.obsidian
            && self.geodes <= inv.geodes
            && self.ore_bots <= inv.ore_bots
            && self.clay_bots <= inv.clay_bots
            && self.obsidian_bots <= inv.obsidian_bots
            && self.geode_bots <= inv.geode_bots
            && !(self.ore == inv.ore
                && self.clay == inv.clay
                && self.obsidian == inv.obsidian
                && self.geodes == inv.geodes
                && self.ore_bots == inv.ore_bots
                && self.clay_bots == inv.clay_bots
                && self.obsidian_bots == inv.obsidian_bots
                && self.geode_bots == inv.geode_bots)
    }

    fn items(&self) -> Items {
        Items(
            self.ore,
            self.clay,
            self.obsidian,
            self.geodes,
            self.ore_bots,
            self.clay_bots,
            self.obsidian_bots,
            self.geode_bots,
        )
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Items(u8, u8, u8, u8, u8, u8, u8, u8);

fn insert(inv: Inventory, stack: &mut Vec<Inventory>, set: &mut HashMap<Items, u8>) {
    let inv = inv.advance_minute();
    if !set.contains_key(&inv.items()) || set.get(&inv.items()) > Some(&inv.minute) {
        stack.push(inv);
        set.insert(inv.items(), inv.minute);
    }
}

fn most_geodes<const MAX: u8>(blueprint: &Blueprint) -> u32 {
    let mut max_geodes = 0;

    let mut set = HashMap::new();
    let mut stack = Vec::new();

    stack.push(Inventory::default());
    set.insert(Inventory::default().items(), 0); // lol technically dont need this but that feels weird

    while let Some(inv) = stack.pop() {
        max_geodes = max_geodes.max(inv.geodes as u32);
        if inv.minute >= MAX {
            continue;
        }

        // if we cant make more geodes than max_geodes at our current state, prune it.
        let remaining = (MAX - inv.minute) as u32;
        if max_geodes
            >= inv.geodes as u32
                + remaining * inv.geode_bots as u32
                + remaining * (remaining - 1) / 2
        {
            continue;
        }

        insert(inv, &mut stack, &mut set);

        if let Some(inv) = inv.try_buy_ore_bot(blueprint) {
            insert(inv, &mut stack, &mut set);
        }

        if let Some(inv) = inv.try_buy_clay_bot(blueprint) {
            insert(inv, &mut stack, &mut set);
        }

        if let Some(inv) = inv.try_buy_obsidian_bot(blueprint) {
            insert(inv, &mut stack, &mut set);
        }

        if let Some(inv) = inv.try_buy_geode_bot(blueprint) {
            insert(inv, &mut stack, &mut set);
        }
    }

    max_geodes
}

pub fn part_1(input: &str) -> impl std::fmt::Display {
    separated_list1(newline, blueprint)(&input)
        .expect("Failed to parse blueprints.")
        .1
        .iter()
        .map(|blueprint| most_geodes::<24>(blueprint) * blueprint.id as u32)
        .sum::<u32>()
        .to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian."
        ).to_string(),
        "33".to_string()
    );
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    separated_list1(newline, blueprint)(&input)
        .expect("Failed to parse blueprints.")
        .1
        .iter()
        .take(3)
        .map(|blueprint| most_geodes::<32>(blueprint))
        .product::<u32>()
        .to_string()
}

#[test]
fn testp2() {
    /*
        assert_eq!(
            part_2(
                "Blueprint 1:
      Each ore robot costs 4 ore.
      Each clay robot costs 2 ore.
      Each obsidian robot costs 3 ore and 14 clay.
      Each geode robot costs 2 ore and 7 obsidian.

    Blueprint 2:
      Each ore robot costs 2 ore.
      Each clay robot costs 3 ore.
      Each obsidian robot costs 3 ore and 8 clay.
      Each geode robot costs 3 ore and 12 obsidian."
                    .to_string()
            ),
            (56*62).to_string()
        );
        */
}
