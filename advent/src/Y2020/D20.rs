use std::collections::HashSet;

#[derive(Clone)]
struct Tile {
    id: u64,
    block: Vec<Vec<bool>>,
}

impl Tile {
    fn new(id: u64, block: Vec<Vec<bool>>) -> Self {
        Self { id, block }
    }

    fn flip(&self) -> Self {
        let mut flip = self.block.clone();
        let len = flip.len();

        for vec in flip.iter_mut() {
            for j in 0..len / 2 {
                vec.swap(j, len - 1 - j);
            }
        }

        Tile::new(self.id, flip)
    }

    fn rotate(&self) -> Self {
        let mut rotate = self.block.clone();

        for i in 0..rotate.len() {
            for j in 0..rotate.len() {
                rotate[i][j] = self.block[rotate.len() - j - 1][i];
            }
        }

        Tile::new(self.id, rotate)
    }

    fn matches_above(&self, other: &Self) -> bool {
        self.block[other.block.len() - 1] == other.block[0]
    }

    fn matches_left(&self, other: &Self) -> bool {
        for i in 0..self.block.len() {
            if self.block[i][other.block.len() - 1] != other.block[i][0] {
                return false;
            }
        }
        true
    }
}

fn part_one(tiles: &Vec<Tile>) -> Tile {
    let size = (tiles.len() as f32).sqrt() as usize;
    let mut all = Vec::new();

    for tile in tiles {
        let mut cur = tile.clone();

        for _ in 0..2 {
            for _ in 0..4 {
                all.push(cur.clone());
                cur = cur.rotate();
            }
            cur = cur.flip();
        }
    }

    let mut grid = vec![vec![Tile::new(0, Vec::new()); size]; size];
    search(0, 0, &mut grid, &mut HashSet::new(), &all);

    let mut block = Vec::new();

    for (i, row) in grid.into_iter().enumerate() {
        for col in row.into_iter() {
            for j in 1..col.block.len() - 1 {
                let ptr = (col.block.len() - 2) * i + j - 1;
                if block.len() <= ptr {
                    block.push(vec![]);
                }

                block[ptr].extend_from_slice(&col.block[j][1..col.block.len() - 1]);
            }
        }
    }

    Tile::new(0, block)
}

fn search(row: usize, col: usize, grid: &mut Vec<Vec<Tile>>, vis: &mut HashSet<u64>, tiles: &Vec<Tile>) {
    if row == grid.len() {
        let len = grid.len();
        vis.insert(0);
        println!("{}", grid[0][0].id * grid[0][len - 1].id * grid[len - 1][0].id * grid[len - 1][len - 1].id);
        return;
    }

    for tile in tiles {
        if vis.contains(&0) { return; }

        if vis.contains(&tile.id) 
        || row > 0 && !grid[row - 1][col].matches_above(tile) 
        || col > 0 && !grid[row][col - 1].matches_left(tile) {
            continue;
        }

        grid[row][col] = tile.clone();
        vis.insert(tile.id);

        if col + 1 == grid.len() {
            search(row + 1, 0, grid, vis, tiles);
        } else {
            search(row, col + 1, grid, vis, tiles);
        }

        vis.remove(&tile.id);
    }
}

fn part_two(mut tile: Tile) {
    let mut ans = i32::MAX;

    for _ in 0..2 {
        for _ in 0..4 {
            let block = &tile.block;
            let expec = block
                .iter()
                .map(|row| row.iter().filter(|&&n| n).count() as i32)
                .sum::<i32>();
            let mut solve = 0;

            for i in 0..block.len() - 2 {
                for j in 0..block.len() - 20 {
                    /*
                    "
                                      # 
                    #    ##    ##    ###
                     #  #  #  #  #  #
                    "
                    */

                    if block[i + 1][j] 
                    && block[i + 2][j+1]

                    && block[i + 2][j+4]
                    && block[i + 1][j+5]
                    && block[i + 1][j+6]
                    && block[i + 2][j+7]

                    && block[i + 2][j+10]
                    && block[i + 1][j+11]
                    && block[i + 1][j+12]
                    && block[i + 2][j+13]

                    && block[i + 2][j+17]
                    && block[i + 1][j+17]
                    && block[i][j+18]
                    && block[i + 1][j+18]
                    && block[i + 1][j+18] {
                        solve += 15;
                    }
                }

                // 2455 too high
                println!("{expec} - {solve} = {}", expec -solve);
                if expec - solve != expec {
                    ans = ans.min(expec - solve);
                }
            }

            tile = tile.rotate();
        }

        tile = tile.flip();
    }

    println!("{ans}");
}

pub fn solve() {
    let inp = std::fs::read_to_string("input/2020/20").unwrap().trim().to_string();
    let inp = inp.split("\n\n").collect::<Vec<_>>();
    let tiles = inp
        .into_iter()
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines.next().unwrap().split(' ').nth(1).unwrap();
            let id = id[..id.len() - 1].parse::<u64>().unwrap();
            Tile::new(id, lines.map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>()).collect())
        })
        .collect::<Vec<_>>();

    let p1 = part_one(&tiles);
    part_two(p1);
}
