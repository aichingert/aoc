use std::collections::{HashSet, HashMap};

fn main() { 
    let inp = std::fs::read_to_string("../input/21").unwrap().trim().to_string();

    let mut allergen: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredients: Vec<Vec<&str>> = Vec::new();

    for line in inp.lines() {
        let (ing, all) = line.split_once(" (contains ").unwrap();
        let (ing, all) = (
            ing.split(' ').collect::<Vec<_>>(), 
            all[..all.len() - 1].split(", ").collect::<Vec<_>>()
        );

        for i in 0..all.len() {
            if let Some(has_to_contain) = allergen.get_mut(all[i]) {
                let list = has_to_contain.clone();
                let list = list.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

                for j in 0..list.len() {
                    if !ing.contains(&list[j]) {
                        has_to_contain.remove(list[j]);
                    }
                }
            } else {
                let mut has_to_contain = HashSet::new();

                for j in 0..ing.len() {
                    has_to_contain.insert(ing[j].to_string());
                }

                allergen.insert(all[i].to_string(), has_to_contain);
            }
        }

        ingredients.push(ing);
    }

    let mut bad = HashSet::new();

    for val in allergen.values() {
        for s in val.iter() {
            bad.insert(s.clone());
        }
    }

    let mut ans = 0;

    for i in 0..ingredients.len() {
        for j in 0..ingredients[i].len() {
            if !bad.contains(ingredients[i][j]) {
                ans += 1;
            }
        }
    }

    let mut mapped: Vec<(String, String)> = Vec::new();

    while allergen.len() > 0 {
        let keys = allergen.clone();
        let keys = keys.keys().collect::<Vec<_>>();
        let mut pos = Vec::new();

        for i in 0..keys.len() {
            if allergen.get(keys[i]).and_then(|pos| if pos.len() == 1 { Some(pos) } else { None }).is_some() {
                pos.push(keys[i]);
            }
        }

        for i in 0..pos.len() {
            let val = allergen.clone();
            let val = val.get(pos[i]).unwrap().iter().collect::<Vec<_>>();
            mapped.push((pos[i].to_string(), val[0].to_string()));

            for k in 0..keys.len() {
                if let Some(ing) = allergen.get_mut(keys[k]) {
                    ing.remove(val[0]);
                }
            }

            allergen.remove(pos[i]);
        }
    }

    mapped.sort_by(|a,b| a.0.cmp(&b.0));

    println!("Part one: {ans}");
    print!("Part two: ");

    for i in 0..mapped.len() - 1 {
        print!("{},", mapped[i].1);
    }

    println!("{}", mapped[mapped.len() - 1].1);
}
