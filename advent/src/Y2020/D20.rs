use std::collections::HashSet;

#[derive(Clone)]
struct Tile {
    id: u64,
    block: Vec<Vec<char>>,
}

impl Tile {
    fn new(id: u64, block: Vec<Vec<char>>) -> Self {
        Self { id, block }
    }

    fn flip(&self) -> Self {
        let mut flip = self.block.clone();
        let len = flip.len();

        for i in 0..len {
            for j in 0..len / 2 {
                flip[i].swap(j, len - 1 - j);
            }
        }

        Tile::new(self.id, flip)
    }

    fn rotate(&self) -> Self {
        let mut rotate = self.block.clone();

        for i in 0..rotate.len() {
            for j in 0..rotate.len() {
                rotate[i][j] = self.block[i][rotate.len() - 1 - j];
            }
        }

        Tile::new(self.id, rotate)
    }

    fn matches_above(&self, other: &Self) -> bool {
        return self.block[other.block.len() - 1] == other.block[0]
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

fn part_one(tiles: &Vec<Tile>) {
    let size = (tiles.len() as f32).sqrt() as usize;

    let mut all = Vec::new();

    for tile in tiles {
        let mut cur = tile.flip();

        for _ in 0..2 {
            for _ in 0..4 {
                all.push(cur.rotate());
            }
            cur = cur.flip();
        }
    }

    search(0, 0, &mut vec![vec![Tile::new(0, Vec::new()); size]; size], &mut HashSet::new(), &all);
}

fn search(row: usize, col: usize, grid: &mut Vec<Vec<Tile>>, vis: &mut HashSet<u64>, tiles: &Vec<Tile>) {
    println!("{row} {col}");
    if row == grid.len() {
        let len = grid.len();
        println!("{}", grid[0][0].id * grid[0][len - 1].id * grid[len - 1][0].id * grid[len - 1][len - 1].id);
    }

    for tile in tiles {
        if vis.contains(&tile.id) {
            continue;
        }
        if row > 0 && !grid[row - 1][col].matches_above(tile) {
            continue;
        }
        if col > 0 && !grid[row][col - 1].matches_left(tile) {
            continue;
        }

        grid[row][col] = tile.clone();
        vis.insert(tile.id);

        if col == grid.len() {
            search(row + 1, 0, grid, vis, tiles);
        } else {
            search(row, col + 1, grid, vis, tiles);
        }

        vis.remove(&tile.id);
    }
}

pub fn solve() {
    let inp = std::fs::read_to_string("input/2020/20").unwrap().trim().to_string();
    let inp = inp.split("\n\n").collect::<Vec<_>>();
    let tiles = inp
        .into_iter()
        .map(|tile| {
            let mut lines = tile.lines();
            let id = lines.next().unwrap().split(' ').skip(1).next().unwrap();
            let id = id[..id.len() - 1].parse::<u64>().unwrap();
            Tile::new(id, lines.map(|l| l.chars().collect::<Vec<_>>()).collect())
        })
        .collect::<Vec<_>>();

    for i in 0..tiles[tiles.len() - 1].block.len() {
        for j in 0..tiles[tiles.len() - 1].block[i].len() {
            print!("{}", tiles[tiles.len() - 1].block[i][j]);
        }
        println!();
    }
    part_one(&tiles);
}
