// Advent of Code 2016, day 21
// (c) aichingert

#[path="../../utils/rust/permutations.rs"] mod permutations;

enum Command {
    SwapPosition(usize,usize),
    SwapLetter(char,char),
    Rotate(bool, usize),
    RotateLetter(char),
    Reverse(usize,usize),
    Move(usize,usize),
}

impl Command {
    fn parse() -> Vec<Command> {
        let mut commands = Vec::<Command>::new();

        for line in std::fs::read_to_string("../input/21").unwrap().lines() {
            let vls = line.split(' ').collect::<Vec<&str>>(); 

            match vls[0] {
                "swap" => commands.push(match vls[1] {
                    "position" => Command::SwapPosition(vls[2].parse().unwrap(),vls[5].parse().unwrap()),
                    "letter" => Command::SwapLetter(vls[2].chars().nth(0).unwrap(),vls[5].chars().nth(0).unwrap()),
                    _ => panic!("invalid swap")
                }),
                "rotate" => match vls[1] {
                    "based" => commands.push(Command::RotateLetter(vls[6].chars().nth(0).unwrap())),
                    _ => commands.push(Command::Rotate(vls[1] == "right", vls[2].parse().unwrap())),
                },
                "reverse" => commands.push(Command::Reverse(vls[2].parse().unwrap(),vls[4].parse().unwrap())),
                "move" => commands.push(Command::Move(vls[2].parse().unwrap(),vls[5].parse().unwrap())),
                _ => panic!("invalid command")
            }
        }

        commands
    }

    fn scramble(&self, password: &mut Vec<char>) {
        match self {
            Command::SwapPosition(x,y) => {
                let ch = password[*x];
                password[*x] = password[*y];
                password[*y] = ch;
            },
            Command::SwapLetter(a,b) => {
                let x = (0..password.len()).find(|&i| &password[i] == a).unwrap();
                let y = (0..password.len()).find(|&i| &password[i] == b).unwrap();
                Command::SwapPosition(x,y).scramble(password);
            },
            Command::Rotate(dir,steps) => for _ in 0..(steps % password.len()) {
                match dir {
                    true => {
                        let ch = password.pop().unwrap();
                        password.insert(0,ch);
                    },
                    false => {
                        let ch = password.remove(0);
                        password.push(ch);
                    },
                }
            },
            Command::RotateLetter(a) => {
                let mut x = (0..password.len()).find(|&i| &password[i] == a).unwrap();
                if x >= 4 { x += 1; }
                Command::Rotate(true,x+1).scramble(password);
            },
            Command::Reverse(x,y) => {
                let rn = &password.clone()[*x..=*y];
                let new = rn.iter().rev().collect::<Vec<&char>>();

                for i in *x..=*y {
                    password[i] = *new[i-*x];
                }
            },
            Command::Move(x,y) => {
                let ch = password.remove(*x);
                password.insert(*y,ch);
            },
        }
    }
}

fn part1(commands: &Vec<Command>) -> String {
    let mut starting: Vec<char> = "abcdefgh".chars().collect::<Vec<char>>();

    for i in 0..commands.len() {
        commands[i].scramble(&mut starting);
    }

    starting.iter().collect::<String>()
}

fn part2(commands: &Vec<Command>) -> String {
    let mut starting: Vec<char> = "abcdefgh".chars().collect::<Vec<char>>();
    let password: Vec<char> = "fbgdceah".chars().collect::<Vec<char>>();
    let perms = permutations::permutations(starting.len(), &mut starting);

    for perm in perms.iter() {
        let mut scrambling = perm.clone();
        for i in 0..commands.len() {
            commands[i].scramble(&mut scrambling);
        }

        if scrambling == password {
            return perm.iter().collect::<String>();
        }
    }

    panic!("no solution")
}

fn main() {
    let commands = Command::parse();

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
}
