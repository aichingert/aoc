use std::{
    path::Path, 
    fs::read_to_string,
    str::FromStr,
    fmt::Debug,
};

pub fn read_to_chars<T: AsRef<Path>>(path: T) -> Vec<char> {
    read_to_string(path)
        .expect("unable to open file")
        .chars()
        .collect()
}

pub fn read_to_slice<T: AsRef<Path>>(path: T, sep: &str) -> Vec<Vec<String>> {
    read_to_string(path)
        .expect("unable to open file")
        .lines()
        .map( | l | 
        l.split(sep)
        .map( | s | s.to_string())
        .collect())
        .collect()
}

pub fn slice<T: AsRef<Path>>(path: T, sep: &str) -> Vec<String> {
    read_to_string(path)
        .expect("unable to open file")
        .split(sep)
        .map( | s | s.trim().to_string())
        .collect()
}

pub fn read_to_numbers<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<U>
where <U as FromStr>::Err: Debug, {
    read_to_string(path)
    .expect("unable to open file")
    .lines()
    .filter( | l | !l.is_empty())
    .map( | l | l.parse::<U>().expect("unable to parse number"))            
    .collect()
}

pub fn numbers<T: AsRef<Path>, U: FromStr>(path: T, sep: &str) -> Vec<Vec<U>> 
where <U as FromStr>::Err: Debug, {
    read_to_string(path)
        .expect("unable to open file")
        .lines()
        .map( | l | l.split(sep)
        .filter( | f | !f.is_empty())
        .map( | v | v.parse::<U>().expect("unable to parse number "))            
        .collect())
        .collect()
}

pub fn read_number_stream<T: AsRef<Path>, U: FromStr>(path: T, sep: &str) -> Vec<U> 
where <U as FromStr>::Err: Debug
{
    read_to_string(path)
        .expect("unable to open file")
        .split(sep)
        .map(|s| s.parse::<U>().expect("invalid input"))
        .collect()
}

pub fn get_chars<T: AsRef<Path>>(path: T) -> Vec<Vec<char>> {
    read_to_string(path)
        .expect("unable to open file")
        .lines()
        .map( | l | l.chars()
        .collect())
        .collect()
}

pub struct Permutations<T> {
    list: Vec<Vec<T>>
}

impl<T: Clone> Permutations<T> {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
    pub fn generate(&mut self, vec: &mut [T], len: usize) {
        if len == 1 {
            self.list.push(Vec::from(vec))
        } else {
            self.generate(vec, len - 1);

            for i in 0..len - 1 {
                if len % 2 == 0 {
                    vec.swap(i, len - 1);
                } else {
                    vec.swap(0, len - 1);
                }

                self.generate(vec, len - 1)
            }
        }
    }
}