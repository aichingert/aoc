use std::collections::HashMap;
use std::cmp::Ordering;

type Blueprint = HashMap<String, (u64, Ingredients)>;
type Ingredients = Vec<(String, u64)>;
type Inventory = HashMap<String, u64>;

fn produce(ore_type: &str, times: u64, blueprint: &Blueprint, inv: &mut Inventory) -> u64 {
    if ore_type == "ORE" {
        return times;
    }

    let needed = if let Some(amount) = inv.remove(ore_type) {
        if amount <= times {
            times - amount
        } else {
            inv.insert(ore_type.to_string(), amount - times);
            return 0;
        }
    } else {
        times
    };

    let mut ore = 0;
    let (gets, ingredients) = blueprint.get(ore_type).unwrap();
    let iterations = (needed as f64 / *gets as f64).ceil() as u64;

    for ingredient in ingredients.iter() {
        ore += produce(&ingredient.0, ingredient.1 * iterations, blueprint, inv);
    }

    let rest = *gets * iterations - needed;
    
    if rest > 0 {
        inv.insert(ore_type.to_string(), rest);
    }

    ore
}

fn main() {
    let mut blueprint: Blueprint = HashMap::new();

    for line in std::fs::read_to_string("../input/14").unwrap().lines() {
        let (needed, creates) = line.split_once(" => ").unwrap();

        let mut ores = needed.split(", ").collect::<Vec<&str>>();

        ores.push(creates);

        let mut ores = ores.iter().map(|ore| {
            let (amount, ore_type) = ore.split_once(' ').unwrap();
            (ore_type.to_string(), amount.parse::<u64>().unwrap())
        }).collect::<Vec<(String, u64)>>();

        let result = ores.pop().unwrap();

        blueprint.insert(result.0, (result.1, ores));
    }

    let part_one: u64 = produce("FUEL", 1, &blueprint, &mut HashMap::new());

    let (mut max, mut min) = (40000000, 0);
    loop {
        let mid = (max + min) / 2;
        let ore = produce("FUEL", mid, &blueprint, &mut HashMap::new());

        match ore.cmp(&1000000000000) {
            Ordering::Less => min = mid + 1,
            Ordering::Greater => max = mid - 1,
            Ordering::Equal => unreachable!(),
        }

        if max < min { 
            break;
        }
    }

    println!("Part one: {}", part_one);
    println!("Part two: {}", (max + min) / 2);
}
