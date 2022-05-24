const INPUT: i32 = 265149;
const SIZE: usize = 20;

fn main() {
    let mut spirale: [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut row: usize = spirale.len() / 2;
    let mut col: usize = spirale.len() / 2;
    let mut found_value: bool = false;

    spirale[row][col] = 1;

    col += 1;
    let mut up: usize = 1;
    let mut left: usize = 2;
    let mut down: usize = 2;
    let mut right: usize = 2;

    while !found_value {

        if !found_value {
            for i in 0..up {
                spirale[row][col] = add_surroundings(&mut spirale, row, col);
    
                if spirale[row][col] > INPUT {
                    found_value = true;
                    break;
                }

                if i < up {
                    row -= 1;
                }
            }
        }

        if !found_value {
            for i in 0..left {
                spirale[row][col] = add_surroundings(&mut spirale, row, col);
    
                if spirale[row][col] > INPUT {
                    found_value = true;
                    break;
                }

                if i < left {
                    col -= 1;
                }
            }
        }

        if !found_value {
            for i in 0..down {
                spirale[row][col] = add_surroundings(&mut spirale, row, col);
    
                if spirale[row][col] > INPUT {
                    found_value = true;
                    break;
                }
    
                if i < down {
                    row += 1;
                }
            }
        }

        if !found_value {
            for _ in 0..right+1 {
                spirale[row][col] = add_surroundings(&mut spirale, row, col);
    
                if spirale[row][col] > INPUT {
                    found_value = true;
                    break;
                }
    
                col += 1;
            }
        }


        up += 2;
        left += 2;
        down += 2;
        right = left;
    }

    println!("{}", spirale[row][col]);
    print_spiral(&spirale);
}

fn add_surroundings(spriale: &mut [[i32; SIZE];SIZE], row: usize, col: usize) -> i32 {
    let mut value: i32 = 0;

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 || row as i32 + i > spriale.len() as i32 || 
            col as i32 + j > spriale.len() as i32 || row as i32 + i < 0 || 
            col as i32 + j < 0 {
                continue;
            }
            
            println!("{}, {}", i, j);
            value += spriale[(row as i32 + i) as usize][(col as i32 + j) as usize];
        }
    }

    value
}

fn print_spiral(spriale: &[[i32; SIZE];SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{} ", spriale[i][j]);
        }
        println!();
    }
}