use std::collections::{HashMap, VecDeque};

#[path = "../../utils/rust/math.rs"]
mod math;
use math::lcm;

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

        keys.push((connections.clone(), if lhs.starts_with("b") {
            lhs.to_string()
        } else {
            v.to_string()
        }));

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

    println!("{:?}", modules.get(&"zh".to_string()));

    // ks: 4021
    // sx: 3877
    // ks: 3851
    // jt: 4049

    println!("{:?}", lcm(lcm(lcm(4021, 3877), 3851), 4049));

    for i in 1..100000 {
        let mut bfs = VecDeque::from([("button".to_string(), "broadcaster".to_string(), false)]);
        while let Some((sender, module, pulse)) = bfs.pop_front() {
            if sender.as_str() == "jt" && module.as_str() == "zh" && pulse {
                println!("{i} yikes");
            }
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

    // 3960000000
}
