use aoc::slice;

use serde_json::Value;

pub struct Aoc2015_12 {
    d: Option<Value>
}

impl Aoc2015_12 {
    pub fn new() -> Self {
        Self { d: None }
    }
}

impl crate::Solution for Aoc2015_12 {
    fn name(&self) -> (usize, usize) {
        (2015, 12)
    }

    fn parse(&mut self) {
        self.d = Some(
            serde_json::from_str(&slice("input/2015/12.txt", "\n")[0])
                .expect("invalid input file")
        );
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(get_sum(self.d.as_ref().unwrap()))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(get_sum_without_red(self.d.as_ref().unwrap()).unwrap())
    }
}

fn get_sum(v: &Value) -> i64 {
    match v {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map( | e | get_sum(e)).sum(),
        Value::Object(o) => o.values().map( | e | get_sum(e)).sum(),
    }
}

fn get_sum_without_red(v: &Value) -> Option<i64> {
    match v {
        Value::Null | Value::Bool(_) => Some(0),
        Value::String(s) => {
            if s == "red" {
                None
            } else {
                Some(0)
            }
        } 
        Value::Number(n) => Some(n.as_i64().unwrap()),
        Value::Array(a) => Some(a.iter().map( | e | get_sum_without_red(e).unwrap_or(0)).sum()),
        Value::Object(o) => {
            let (somes, nones): (Vec<_>, Vec<_>) =
                o.values().map(get_sum_without_red).partition(Option::is_some);

            if nones.is_empty() {
                Some(somes.iter().map(|entry| entry.unwrap()).sum())
            } else {
                Some(0)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let v = serde_json::from_str(r#"{"a":2,"b":4}"#).expect("failed to parse json");
        assert_eq!(6, get_sum(&v), "wrong sum");
    }
}