# library

## Reads the input to a Vec<char>
´´´ rust
pub fn read_to_chars<T: AsRef<Path>>(path: T) -> Vec<char>
´´´

## Reads the input to a Vec<String>
´´´ rust
pub fn slice<T: AsRef<Path>>(path: T, sep: &str) -> Vec<String>
´´´

## Reads the input to a Vec<U>, where u is the type you pass to the fn
´´´ rust
pub fn read_to_numbers<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<U>
where <U as FromStr>::Err: Debug,
´´´

## Reads the input to a Vec<Vec<U>>, where u is the type you pass to the fn
´´´ rust
pub fn numbers<T: AsRef<Path>, U: FromStr>(path: T, sep: char) -> Vec<Vec<U>> 
where <U as FromStr>::Err: Debug,
´´´