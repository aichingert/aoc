use std::collections::HashMap;

type Blueprint = HashMap<String, (u32, Ingredients)>;
type Ingredients = Vec<(String, u32)>;
type Inventory = HashMap<String, u32>;

fn produce(ore_type: &str, times: u32, blueprint: &Blueprint, inv: &mut Inventory) -> u32 {
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
    let mut created = 0;
    let (gets, ingredients) = blueprint.get(ore_type).unwrap();

    for _ in 0..(needed as f32 / *gets as f32).ceil() as u32 {
        created += *gets;
        for ingredient in ingredients.iter() {
            ore += produce(&ingredient.0, ingredient.1, blueprint, inv);
        }
    }

    let rest = created - needed;
    
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
            (ore_type.to_string(), amount.parse::<u32>().unwrap())
        }).collect::<Vec<(String, u32)>>();

        let result = ores.pop().unwrap();

        blueprint.insert(result.0, (result.1, ores));
    }

    println!("{:?}", produce("FUEL", 1, &blueprint, &mut HashMap::new()));
}
