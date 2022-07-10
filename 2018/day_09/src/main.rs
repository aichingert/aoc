fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve(&input);
}

fn solve(input: &String) {
    let split: Vec<&str> = input.split(' ').collect();

    let amount_of_players: usize = split[0].parse::<usize>().unwrap();
    let amount_of_marbles: usize = split[6].parse::<usize>().unwrap();

    solve_parts(amount_of_players, amount_of_marbles, false);
    solve_parts(amount_of_players, amount_of_marbles * 100, true);
}

fn solve_parts(amount_of_players: usize, amount_of_marbles: usize, part: bool) {
    let mut circle: Vec<usize> = vec![0];
    let mut players: Vec<usize> = vec![0; amount_of_players as usize];
    let mut current_player: usize;
    let mut current_position: usize = 0;
    let mut marble_value: usize;
    
    for i in 1..=amount_of_marbles {
        if i % 23 == 0 {
            if current_position as i32 - 7 > 0 {
                current_position -= 7;
            } else {
                current_position = circle.len() - 7 + current_position;
            }

            if current_position == circle.len() - 1 {
                marble_value = circle.remove(current_position);
                current_position -= 1;
            } else {
                marble_value = circle.remove(current_position);
            }

            current_player = i % amount_of_players;
            players[current_player as usize] += marble_value + i;
        } else {
            current_position += 2;

            if current_position > circle.len() {
                current_position = 1;
            }

            circle.insert(current_position, i);
        }
    }

    let mut highscore: usize = 0;

    for i in 0..players.len() {
        if players[i] > highscore {
            highscore = players[i];
        }
    }

    if !part {
        println!("Solution part one: {}", highscore);
    } else {
        println!("Solution part two: {}", highscore);
    }
}