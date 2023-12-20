use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug)]
enum Module {
    Bc,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

fn main() {
    let inp = std::fs::read_to_string("../input/20").unwrap().trim().to_string();
    let mut modules = HashMap::new();

    let mut keys = Vec::new();

    for line in inp.lines() {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();
        let (t, v) = (&lhs[..1], &lhs[1..]);
        let connections = rhs.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

        if lhs.starts_with("b") {
            keys.push((connections.clone(), lhs.to_string()));
        } else {
            keys.push((connections.clone(), v.to_string()));
        }

        match (t, v) {
            ("b", "roadcaster") => modules.insert(lhs.to_string(), (Module::Bc, connections)),
            ("%", _) => modules.insert(v.to_string(), (Module::FlipFlop(false), connections)),
            ("&", _) => modules.insert(v.to_string(), (Module::Conjunction(HashMap::new()), connections)),
            _ => None,
        };
    }

    for i in 0..keys.len() {
        for conns in &keys[i].0 {
            if let Some((module_t, _)) = modules.get_mut(conns) {
                match module_t {
                    Module::Conjunction(cache) => cache.insert(keys[i].1.clone(), false),
                    _ => None,
                };
            }
        }
    }

    let (mut lo, mut hi) = (0, 0);

    'inf: for i in 0..1000 {
        let mut bfs = VecDeque::from([("button".to_string(), "broadcaster".to_string(), false)]);

        while let Some((sender, module, pulse)) = bfs.pop_front() {
            if module.as_str() == "rx" && !pulse {
                println!("SOL {i}");
                break 'inf;
            }

            match pulse {
                false => lo += 1,
                true => hi += 1,
            };

            let result = modules.get_mut(&module);
            if result.is_none() {
                continue;
            }

            let (module_t, connections) = result.unwrap();

            match module_t {
                Module::Bc => connections.iter().for_each(|conn| bfs.push_back((module.clone(), conn.to_string(), pulse))),
                Module::FlipFlop(state) => {
                    if pulse { continue; }
                    *state = !*state;

                    connections.iter().for_each(|conn| bfs.push_back((module.clone(), conn.to_string(), *state)));
                }
                Module::Conjunction(cache) => {
                    *cache.get_mut(&sender).unwrap() = pulse;

                    let send = !cache.iter().all(|val| *val.1);
                    connections.iter().for_each(|conn| bfs.push_back((module.clone(), conn.to_string(), send)));
                }
            }
        }
    }

    let mut cur = VecDeque::from(["rx".to_string()]);
    let mut sen = HashSet::new();
    let mut x = 0;

    while let Some(el) = cur.pop_front() {
        if !sen.insert(el.clone()) {
            continue;
        }
        for module in modules.iter() {
            let (name, (module_t, conn)) = module;
            
            match module_t {
                Module::Conjunction(cache) => {
                    if conn.contains(&el) {
                        println!("{name}[{module_t:?}]: ");

                        for con in cache {
                            println!("  {:?}", con.0);
                            cur.push_back(con.0.clone());
                        }
                    }
                }
                _ => {
                    if conn.contains(&el) {
                        println!("{name}[{module_t:?}]: ");

                        for con in conn {
                            println!("  {:?}", con);
                        }
                    }
                }
            }
        }
    }
    
    println!("hi: {hi} - lo: {lo} {}", hi * lo);
}
