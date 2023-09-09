pub fn answer() -> (u32, u32) {
    let (list, poss) = parse();
    let mut cnt = 0;
    solve(&list, 0, &String::new(), &Vec::new(), &poss, &mut cnt);

    println!("{cnt}");
    (0, 0)
}

fn solve(list: &Vec<Value>, cur: usize, value: &String, rest: &Vec<usize>, pos: &Vec<String>, cnt: &mut u32) {
    let mut m_value;
    let mut m_rest;

    match &list[cur] {
        Value::Pointers(ptr) => {
            'outer: for i in 0..ptr.len() {
                m_value = value.clone();
                m_rest = rest.clone();

                for j in 0..ptr[i].len() {
                    let pt = ptr[i][j];

                    match &list[pt] {
                        Value::Lit(s) => {
                            m_value.push_str(&s.clone());
                            if j + 1 < ptr[i].len() {
                                continue;
                            } else {
                                while !m_rest.is_empty() {
                                    let first = m_rest.remove(0);

                                    match &list[first] {
                                        Value::Lit(s) => m_value.push_str(&s.clone()),
                                        Value::Pointers(_) => {
                                            solve(list, first, &m_value, &m_rest, pos, cnt);
                                            continue 'outer;
                                        },
                                    }
                                }
                            }
                        }
                        Value::Pointers(_) => {
                            for k in j+1..ptr[i].len() {
                                m_rest.insert(0, ptr[i][k]);
                            }
                            solve(list, pt, &m_value, &m_rest, pos, cnt);
                            break;
                        },
                    }

                    if pos.contains(&m_value) {
                        *cnt += 1;
                    }
                }
            }
        },
        _ => panic!("how {cur} {:?}", rest),
    };
}

#[derive(Debug)]
enum Value {
    Lit(String),
    Pointers(Vec<Vec<usize>>),
}

fn parse() -> (Vec<Value>, Vec<String>) {
    let mut list = Vec::<Value>::new();

    let binding = std::fs::read_to_string("../input/19").unwrap();
    let (rules, input) = binding.split_once("\n\n").unwrap();

    'outer: for line in rules.lines() {
        let line = line.replace("\"", "");
        let (_, values) = line.split_once(": ").unwrap();

        let mut storing = Vec::new();
        let possibilities: Vec<&str> = values.split(" | ").collect();

        for possibility in possibilities {
            let x = possibility.split(' ').collect::<Vec<&str>>();

            if x.len() == 1 && x[0].parse::<usize>().is_err() {
                list.push(Value::Lit(x[0].to_string()));
                continue 'outer;
            } else {
                storing.push(x.iter().map(|s| s.parse::<usize>().unwrap()).collect());
            }
        }


        list.push(Value::Pointers(storing));
    }

    (list, input.split('\n').map(|s| s.to_string()).collect::<Vec<String>>())
}
