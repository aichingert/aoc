const SIZE: usize = 5;

type Board = [[(u32, bool); SIZE]; SIZE];

fn is_board_winning(i: usize, j: usize, board: &Board) -> bool {
    let (mut is_winning_h, mut is_winning_v) = (true, true);
    for k in 0..SIZE {
        if !board[i][k].1 {
            is_winning_v = false;
        }
        if !board[k][j].1 {
            is_winning_h = false;
        }
    }

    is_winning_h || is_winning_v
}

fn final_score(num: u32, board: &Board) -> u32 {
    num * board.iter().map(|arr| arr.iter().map(|n| if !n.1 { n.0 } else { 0 }).sum::<u32>()).sum::<u32>()
}

fn solve(nums: &[u32], mut boards: Vec<Board>) -> (u32, u32) {
    let mut removed = Vec::new();

    for num in nums.iter() {
        let mut i = 0;
        'outer: while i < boards.len() {
            for j in 0..SIZE {
                for k in 0..SIZE {
                    if boards[i][j][k].0 == *num {
                        boards[i][j][k].1 = true;

                        if is_board_winning(j, k, &boards[i]) {
                            removed.push(final_score(*num, &boards.remove(i)));
                            continue 'outer;
                        }
                    }
                }
            }

            i += 1;
        } 
    }

    (*removed.first().unwrap(), *removed.last().unwrap())
}

fn parse() -> (Vec<u32>, Vec<Board>) {
    let inp = std::fs::read_to_string("../input/04").unwrap().trim().to_string();
    let (numbers, boards) = inp.split_once("\n\n").unwrap();
    let (numbers, boards_str) = (
        numbers.split(',').map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>(),
        boards.split("\n\n").collect::<Vec<_>>()
    );

    (numbers, boards_str.iter().map(|s| {
        let mut cur: Board = [[(0, false); 5]; 5];
        s.lines().collect::<Vec<_>>().into_iter().enumerate().for_each(|(i, line)| {
            line.split(' ').filter(|n| !n.is_empty()).collect::<Vec<_>>().into_iter().enumerate().for_each(|(j, e)| { 
                cur[i][j].0 = e.parse::<u32>().unwrap(); 
            })
        });
        cur
    }).collect::<Vec<Board>>())
}

fn main() {
    let (numbers, boards) = parse();
    let (part_one, part_two) = solve(&numbers, boards);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
