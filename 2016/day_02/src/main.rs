use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Error, couldn't open file!");
    
    let mut pos_vertical: i32 = 2;
    let mut pos_horizontal: i32 = 0;
    let mut keypad: [[String; 5]; 5] = Default::default();
    keypad[0][2] = "1".to_string();
    keypad[1][1] = "2".to_string();
    keypad[1][2] = "3".to_string();
    keypad[1][3] = "4".to_string();
    keypad[2][0] = "5".to_string();
    keypad[2][1] = "6".to_string();
    keypad[2][2] = "7".to_string();
    keypad[2][3] = "8".to_string();
    keypad[2][4] = "9".to_string();
    keypad[3][1] = "A".to_string();
    keypad[3][2] = "B".to_string();
    keypad[3][3] = "C".to_string();
    keypad[4][2] = "D".to_string();

    print!("Number: ");

    for line in content.lines() {
        for c in line.chars() {
            match c {
                'U' => if hits_edge(pos_vertical - 1, pos_horizontal)  { pos_vertical -= 1 } else {  },
                'D' =>  if hits_edge(pos_vertical + 1, pos_horizontal)  { pos_vertical += 1 } else {  },
                'R' =>  if hits_edge(pos_vertical, pos_horizontal + 1)  { pos_horizontal += 1 } else {  },
                'L' =>  if hits_edge(pos_vertical, pos_horizontal - 1) { pos_horizontal -= 1 } else {  },
                _ => panic!(),
            };
        }

        print!("{}", keypad[(pos_vertical) as usize][(pos_horizontal) as usize]);
    }

    println!();

    for i in 0..keypad.len() {
        for j in 0..keypad.len() {
            if keypad[i][j] != "" { print!("{}", keypad[i][j]); } else { print!(" "); }
        }
        println!();
    }
}

fn hits_edge(vert: i32, horizontal: i32) -> bool {
    if vert < 0 || horizontal < 0 || vert > 4 || horizontal > 4 {
        return false;
    }

    if vert == 0 && horizontal == 1 || vert == 0 && horizontal == 3 {
        return false
    }

    if  vert == 1 && horizontal == 0 || vert == 1 && horizontal == 4 {
        return false
    }

    if  vert == 3 && horizontal == 0 || vert == 3 && horizontal == 4 {
        return false
    }

    if vert == 4 && horizontal == 1 || vert == 4 && horizontal == 3 {
        return false
    }

    true
}


/* Part 1
 let mut numbers: [[u8; 3]; 3] = [[0; 3]; 3];
    let mut count = 1;
    let mut pos_vertical: i32 = 1;
    let mut pos_horizontal: i32 = 1;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            numbers[i][j] = count;
            count += 1;
        }
    }

    println!("{:?}", numbers);
    print!("Number: ");

    for line in content.lines() {
        for c in line.chars() {
            match c {
                'U' => if (pos_vertical - 1) > -1  { pos_vertical -= 1 } else {  },
                'D' =>  if (pos_vertical + 1) < 3  { pos_vertical += 1 } else {  },
                'R' =>  if (pos_horizontal + 1) < 3  { pos_horizontal += 1 } else {  },
                'L' =>  if (pos_horizontal - 1) > -1  { pos_horizontal -= 1 } else {  },
                _ => panic!(),
            };
        }

        print!("{}", numbers[(pos_vertical) as usize][(pos_horizontal) as usize]);
    }
*/