use std::collections::{HashSet, HashMap};

fn part_one(allergen: &HashMap<String, Vec<String>>, ingredients: &Vec<Vec<String>>) -> u32 {
    let mut bad = HashSet::new();
    allergen.values().for_each(|vec| vec.iter().for_each(|s| { bad.insert(s); }));

    ingredients
        .iter()
        .map(|line| {
            line.iter().filter(|s| !bad.contains(s)).count() as u32
        })
        .sum::<u32>()
}

fn part_two(mut allergen: HashMap<String, Vec<String>>) -> String {
    let mut allergen_in_ingredient: Vec<(String, String)> = Vec::new();

    while allergen.len() > 0 {
        let keys = allergen.keys().filter(|&k| allergen[k].len() == 1).map(|s| s.to_string()).collect::<Vec<_>>();

        for i in 0..keys.len() {
            let current = allergen.remove(&keys[i]).unwrap();

            for value in allergen.values_mut() {
                if let Some(p) = value.iter().position(|x| x == &current[0]) {
                    value.remove(p);
                }
            }

            allergen_in_ingredient.push((keys[i].to_string(), current[0].clone()));
        }
    }

    allergen_in_ingredient.sort_by(|a,b| a.0.cmp(&b.0));
    let mut s = String::new();

    for ingredient in allergen_in_ingredient {
        s.push_str(&format!("{},", ingredient.1));
    }
    s.pop();

    s
}

fn parse() -> (HashMap<String, Vec<String>>, Vec<Vec<String>>) {
     let inp = std::fs::read_to_string("../input/21").unwrap().trim().to_string();

    let mut allergen: HashMap<String, Vec<String>> = HashMap::new();
    let mut ingredients: Vec<Vec<String>> = Vec::new();

    for line in inp.lines() {
        let (ing, all) = line.split_once(" (contains ").unwrap();
        let (ing, all) = (
            ing.split(' ').map(|s| s.to_string()).collect::<Vec<_>>(), 
            all[..all.len() - 1].split(", ").collect::<Vec<_>>()
        );

        for i in 0..all.len() {
            if let Some(has_to_contain) = allergen.get_mut(all[i]) {
                let mut j = 0;

                while j < has_to_contain.len() {
                    if !ing.contains(&has_to_contain[j]) {
                        has_to_contain.remove(j);
                    } else {
                        j += 1;
                    }
                }
            } else {
                allergen.insert(all[i].to_string(), ing.clone());
            }
        }

        ingredients.push(ing);
    }

    (allergen, ingredients)
}

fn main() { 
    let (allergen, ingredients) = parse();

    println!("Part one: {}", part_one(&allergen, &ingredients));
    println!("Part two: {}", part_two(allergen));
}
