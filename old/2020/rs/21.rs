use std::collections::{HashMap, HashSet};
/*
    mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)

    mxmxvkd: dairy, fish
    kfcds:  
    sqjhc:  fish
    nhms:   

    trh: 
    fvjkl: 
    sbzzf: 
    mxmxvkd: dairy

    sqjhc: soy
    fvjkl: soy

    sqjhc: fish
    mxmxvkd: fish
    sbzzf: 

    kfcds, nhms, trh, sbzzf, 
*/

fn main() { 
    let inp = std::fs::read_to_string("../input/21").unwrap().trim().to_string();
    let inp = inp.lines().collect::<Vec<_>>();

    // words - allergens of a line
    let mut lines = Vec::new();
    let mut words: HashMap<String, HashSet<&str>> = HashMap::new();

    for i in 0..inp.len() {
        let (ing, all) = inp[i].split_once(" (contains ").unwrap();
        let (ing, all) = (
            ing.split(' ').collect::<Vec<_>>(), 
            all[..all.len() - 1].split(", ").collect::<Vec<_>>()
        );

        for i in 0..ing.len() {
            words.entry(ing[i].to_string())
                .and_modify(|allergens| {
                    for j in 0..all.len() {
                        allergens.insert(all[j]);
                    }
                })
                .or_insert({
                    let mut allergens = HashSet::new();
                    for j in 0..all.len() {
                        allergens.insert(all[j]);
                    }
                    allergens
                });
        }

        lines.push((ing, all));
    }

    println!("{:?}", lines[0]);

    //println!("{:?}", inp);
}
