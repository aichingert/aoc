# library

## Parsing
### Reads the input to a Vec<char>
``` rust
pub fn read_to_chars<T: AsRef<Path>>(path: T) -> Vec<char>
```

### Reads the input to a Vec<Vec<char>>
``` rust
pub fn get_chars<T: AsRef<Path>>(path: T) -> Vec<Vec<char>>
```

### Reads the input to a Vec<Vec<String>>
``` rust
pub fn read_to_slice<T: AsRef<Path>>(path: T, sep: &str) -> Vec<Vec<String>> {
```

### Reads the input to a Vec<String>
``` rust
pub fn slice<T: AsRef<Path>>(path: T, sep: &str) -> Vec<String>
```

### Reads the input to a Vec<Vec<U>>, where u is the type you pass to the fn
``` rust
pub fn numbers<T: AsRef<Path>, U: FromStr>(path: T, sep: char) -> Vec<Vec<U>> 
where <U as FromStr>::Err: Debug,
```

### Reads the input to a Vec<Vec<U>>, where U is the type you pass to the fn
``` rust
pub fn read_to_map<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<Vec<U>> 
where <U as FromStr>::Err: Debug, Vec<U>: FromIterator<u8>
```

### Reads the input to a Vec<U>, where u is the type you pass to the fn
``` rust
pub fn read_to_numbers<T: AsRef<Path>, U: FromStr>(path: T) -> Vec<U>
where <U as FromStr>::Err: Debug,
```

### Reads the input to a Vec<U>, where u is the type you pass to the fn
``` rust
pub fn read_number_stream<T: AsRef<Path>, U: FromStr>(path: T, sep: &str) -> Vec<U> 
where <U as FromStr>::Err: Debug
```

### Struct Permutations gets the permutations of a vec
``` rust 
pub struct Permutations<T>
```

## Math
### gcd - greatest common divisor
```rust
pub gcd(a: usize, b: usize) -> usize
```