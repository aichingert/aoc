use std::{
    path::Path, 
    fs::read_to_string,
    str::FromStr,
    fmt::Debug,
    ops::{Div, Mul, Sub}
};


/// Returns the inputx^x
/// as a list of chars
///
pub fn read_to_chars<T: AsRef<Path>>(path: T) -> Vec<char> {
    read_to_string(path)
        .expect("unable to open file")
        .chars()
        .filter(|f| *f != '\n')
        .collect()
}

/// Returns a list of a
/// list of seperated strings
///
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

/// Returns a list of Strings that 
/// are seperated
/// by a seperater 'sep'
///
pub fn slice<T: AsRef<Path>>(path: T, sep: &str) -> Vec<String> {
    read_to_string(path)
        .expect("unable to open file")
        .split(sep)
        .map( | s | s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Expects the input to be
/// a single number per line
/// and then parses that into
/// a list
///
pub fn read_to_numbers<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<U>
where <U as FromStr>::Err: Debug, {
    read_to_string(path)
    .expect("unable to open file")
    .lines()
    .filter( | l | !l.is_empty())
    .map( | l | l.trim().parse::<U>().expect("unable to parse number"))
    .collect()
}

/// Expects the input to be a map of numbers
/// Returns it as a list of lists
///
pub fn read_to_map<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<Vec<U>>
where <U as FromStr>::Err: Debug, Vec<U>: FromIterator<u8> {
    read_to_string(path)
        .expect("unable to open file")
        .lines()
        .map(|l| l.chars()
            .map(|c| (c as u8 - '0' as u8))
            .collect::<Vec<U>>()
        )
        .collect::<Vec<Vec<U>>>()
}


/// Expects the input to be
/// a map of numbers only
/// and then parses it per
/// line per character 
///
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

/// Expects the input to be
/// a line of numbers only
/// and then parses it per
/// character into a number
///
pub fn read_number_stream<T: AsRef<Path>, U: FromStr>(path: T, sep: &str) -> Vec<U> 
where <U as FromStr>::Err: Debug
{
    read_to_string(path)
        .expect("unable to open file")
        .split(sep)
        .map(|s| s.trim().parse::<U>().expect("invalid input"))
        .collect()
}

/// Gives the input line
/// per line parsed per
/// character
///
pub fn get_chars<T: AsRef<Path>>(path: T) -> Vec<Vec<char>> {
    read_to_string(path)
        .expect("unable to open file")
        .lines()
        .map( | l | l.chars()
        .collect())
        .collect()
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut f = a;
    let mut s = b;

    while s > 0 {
        let r = f % s;
        f = s;
        s = r;
    }

    f
}

/// Permutations 
/// 
/// calculates the possible
/// permutations for the given
/// list
///
pub struct Permutations<T> {
    pub list: Vec<Vec<T>>
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
