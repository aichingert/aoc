#[derive(Debug, Clone)]
struct Tower {
    name: String,
    holding: Vec<String>,
    weigth: i32,
}

impl Tower {
    fn new(_name: String, _holding: Vec<String>, _weight: i32) -> Tower{
        Tower {
            name: _name, 
            holding: _holding, 
            weigth: _weight 
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("err");
    let lines: Vec<_> = contents.split("\r").collect();
    let mut stores_towers: bool = false;
    let mut towers: Vec<Tower> = Vec::new();
    let mut stored: Vec<String> = Vec::new();
    let mut bottom: Tower = Tower::new("".to_string(), Vec::new(), 0);

    for line in lines {
        let split: Vec<_> = line.split("->").collect();
        let mut split_tower_names: Vec<_> = Vec::new();

        if split.len() > 1 {
            split_tower_names = split[1].split(',').collect();
            stores_towers = true;
        }

        if stores_towers {
            let mut storing: Vec<String> = Vec::new();
            for name in split_tower_names {
                storing.push(name.trim().to_string());
            }

            let tower_name: Vec<_> = split[0].split(' ').collect();
            let mut string_weight: String = tower_name[1].to_string();
            let mut weight: String = String::from("");

            while string_weight.len() > 0 {
                let popd = string_weight.pop().unwrap();

                if popd.is_numeric() {
                    weight.push(popd);
                }
            }
            
            weight = reverse(&mut weight);

            let tower: Tower = Tower::new(tower_name[0].trim().to_string(), storing, weight.parse().unwrap());

            towers.push(tower);
        }
        else {
            let tower_name: Vec<_> = line.split(' ').collect();

            let mut string_weight: String = tower_name[1].to_string();
            let mut weight: String = String::from("");

            while string_weight.len() > 0 {
                let popd = string_weight.pop().unwrap();

                if popd.is_numeric() {
                    weight.push(popd);
                }
            }

            weight = reverse(&mut weight);

            let tower: Tower = Tower::new(tower_name[0].trim().to_string(), Vec::new(), weight.parse().unwrap());

            towers.push(tower);   
        }
    }

    for tower in towers.clone() {
        if tower.holding.len() < 1 {
            stored.push(tower.name);
        }
        else {
            for disc_tower in tower.holding {
                stored.push(disc_tower);
            }
        }
    }

    for i in 0..towers.len() {
        if !stored.contains(&towers[i].name) {
            bottom = Tower::new(towers[i].name.clone(), towers[i].holding.clone(), towers[i].weigth);
        }
    }

    let mut sums: Vec<i32> = Vec::new();

    for branch in &bottom.holding {
        for tower in &towers {
            if *branch == tower.name {
                sums.push(get_real_weight(&towers, &tower));
            }
        }
    }

    go_to_wrong_sum(&mut sums, &towers, &bottom);

    println!("{:?}", sums);
}

fn go_to_wrong_sum(sums: &mut Vec<i32>, all_towers: &Vec<Tower>, root: &Tower) {
    sums.clear();
    let mut idx: usize = 0;
    let mut found_wrong_sum: bool = false;
    println!("{}", root.weigth);

    for branch in &root.holding {
        for tower in all_towers {
            if branch == &tower.name {
                sums.push(get_real_weight(all_towers, &tower));
            }
        }
    }

    for sum in sums.clone() {
        sums.remove(idx);
        if !sums.contains(&sum) {
            println!("{} {:?}", sum, sums);
            sums.insert(idx, sum);
            found_wrong_sum = true;
            break;
        } else {
            sums.insert(idx, sum);
            idx += 1;
        }
    }

    if found_wrong_sum {
        for tower in all_towers {
            if root.holding[idx] == tower.name {
                println!("{}", root.holding[idx]);
                go_to_wrong_sum(sums, all_towers, tower);
                break;
            }
        }
    }
}

fn get_real_weight(all_towers: &Vec<Tower>, root: &Tower) -> i32 {
    let mut weigth: i32 = root.weigth;

    if root.holding.len() != 0 {
        for branch in root.holding.clone() {
            for tower in all_towers {
                if branch == tower.name {
                    weigth += get_real_weight(all_towers, tower);
                }
            }
        }
    }

    weigth
}

fn reverse(to_reverse: &mut String) -> String {
    let mut returning: String = String::from("");
    let mut poped: Vec<char> = Vec::new();

    while to_reverse.len() > 0 {
        poped.insert(poped.len(), to_reverse.pop().unwrap());
    }

    for chr in poped {
        returning.push(chr);
    }

    returning
}