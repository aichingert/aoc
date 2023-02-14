// Advent of Code 2015, day 23
// (c) aichingert

use std::collections::HashMap;

fn computer(instr: &[Instr], part: bool) -> i32 {
    let mut registers: HashMap<String, i32> = match part {
        true => HashMap::from([("a".to_string(),0),("b".to_string(),0)]),
        false => HashMap::from([("a".to_string(),1),("b".to_string(),0)]),
    };
    let mut loc: usize = 0;

    while loc < instr.len() {
        match &instr[loc] {
            Instr::Hlf(reg) => if let Some(value) = registers.get_mut(&reg[..]) { *value /= 2; },
            Instr::Tpl(reg) => if let Some(value) = registers.get_mut(&reg[..]) { *value *= 3; },
            Instr::Inc(reg) => if let Some(value) = registers.get_mut(&reg[..]) { *value += 1; },
            Instr::Jmp(offset) => { loc = (loc as isize + offset) as usize; continue },
            Instr::Jie(reg,offset) => if registers[&reg[..]] & 1 == 0 { loc = (loc as isize + offset) as usize; continue },
            Instr::Jio(reg,offset) => if registers[&reg[..]] == 1 { loc = (loc as isize + offset) as usize; continue },
        }
        loc += 1;
    }
    
    registers["b"]
}

enum Instr<'a> {
    Hlf(&'a str),
    Tpl(&'a str),
    Inc(&'a str),
    Jmp(isize),
    Jie(&'a str, isize),
    Jio(&'a str, isize)
}

fn main() {
    let inp = std::fs::read_to_string("../input/23").unwrap();
    let mut instr = Vec::new();

    for line in inp.lines() {
        let values = line.split(' ').collect::<Vec<&str>>();
    
        instr.push(match values[0] {
            "hlf" => Instr::Hlf(values[1]),
            "tpl" => Instr::Tpl(values[1]),
            "inc" => Instr::Inc(values[1]),
            "jmp" => Instr::Jmp(values[1].parse::<isize>().unwrap()),
            "jie" => Instr::Jie(&values[1][..values[1].len()-1], values[2].parse::<isize>().unwrap()),
            "jio" => Instr::Jio(&values[1][..values[1].len()-1], values[2].parse::<isize>().unwrap()),
            _ => panic!("invalid input")
        });
    }

    println!("Part 1: {}", computer(&instr, true));
    println!("Part 2: {}", computer(&instr, false));
}
