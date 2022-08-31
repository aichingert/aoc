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

pub fn slice<T: AsRef<Path>>(path: T, sep: &str) -> Vec<String> {
    read_to_string(path)
        .expect("unable to open file")
        .split(sep)
        .map( | s | s.to_string())
        .collect()
}

pub fn numbers<T: AsRef<Path>, U: FromStr>(path: T, sep: char) -> Vec<Vec<U>> 
where <U as FromStr>::Err: Debug,
{
    read_to_string(path)
        .expect("unable to open file")
        .lines()
        .map( | l | l.split(sep)
        .map( | v | v.parse::<U>().expect("unable to parse number"))            
        .collect())
        .collect()
}