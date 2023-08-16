use std::collections::HashMap;

type Map = HashMap<String, Vec<Vec<String>>>;

pub fn answer() -> (u32, u32) {
    parse();

    (0, 0)
}

#[derive(Debug)]
enum Value {
    Literal(String),
    List(Vec<Box<Value>>),
}

fn uncover(m: &Map, ptr: &String) -> Vec<Box<Value>> {
    let mut values: Vec<Box<Value>> = Vec::new();

    match m.get(ptr) {
        Some(options) => {
            for option in options.iter() {
                let mut store = Vec::new();
                for element in option.iter() {
                    store.push(Box::new(Value::List(uncover(m, element))));
                }
                values.push(Box::new(Value::List(store)));
            }
        }
        None => values.push(Box::new(Value::Literal(ptr.clone()))),
    }

    values
}

fn open(value: &Value) {
    match value {
        Value::Literal(s) => {
            print!("{s} ");
        }
        Value::List(l) => {
            for e in l.iter() {
                open(e);
            }
            print!(" ");
        }
    }
}

fn parse() -> Map {
    let mut mappings: Map = HashMap::new();

    for line in std::fs::read_to_string("../input/19").unwrap().lines() {
        let line = line.replace("\"", "");
        let (key, values) = line.split_once(": ").unwrap();

        let list: Vec<&str> = values.split(" | ").collect();

        for element in list.iter() {
            mappings
                .entry(key.to_string())
                .or_insert(Vec::new())
                .push(element.split(' ').map(|e| e.to_string()).collect());
        }
    }

    let x = uncover(&mappings, &"0".to_string());

    println!("{:?}", x[0]);
    for v in x.iter() {
        open(v);
    }

    mappings
}
