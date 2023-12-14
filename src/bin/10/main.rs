#[derive(Debug)]
enum Tile {
    Vertical,
    Horiztonal,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start,
}

impl Tile {
    fn char_to_tile(tile: char) -> Tile {
        match tile {
            '|' => Tile::Vertical,
            '-' => Tile::Horiztonal,
            'L' => Tile::NE,
            'J' => Tile::NW,
            'F' => Tile::SE,
            '7' => Tile::SW,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Unknown tile {tile}"),
        }
    }

    fn connected(&self) -> Vec<RelativePosition> {
        match self {
            Tile::Vertical => vec![
                RelativePosition { x: 0, y: 1 },
                RelativePosition { x: 0, y: -1 },
            ],
            Tile::Horiztonal => vec![
                RelativePosition { x: -1, y: 0 },
                RelativePosition { x: 1, y: 0 },
            ],

            Tile::NE => vec![
                RelativePosition { x: 0, y: -1 },
                RelativePosition { x: 1, y: 0 },
            ],

            Tile::NW => vec![
                RelativePosition { x: 0, y: -1 },
                RelativePosition { x: -1, y: 0 },
            ],
            Tile::SE => vec![
                RelativePosition { x: 0, y: 1 },
                RelativePosition { x: 1, y: 0 },
            ],
            Tile::SW => vec![
                RelativePosition { x: 0, y: 1 },
                RelativePosition { x: -1, y: 0 },
            ],
            Tile::Ground => vec![],
            Tile::Start => vec![
                RelativePosition { x: 0, y: 1 },
                RelativePosition { x: 0, y: -1 },
                RelativePosition { x: 1, y: 0 },
                RelativePosition { x: -1, y: 0 },
            ],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct RelativePosition {
    x: isize,
    y: isize,
}

impl std::ops::Add<&RelativePosition> for &Position {
    type Output = Position;

    fn add(self, rhs: &RelativePosition) -> Self::Output {
        Self::Output {
            x: self.x.checked_add_signed(rhs.x).unwrap_or_default(),
            y: self.y.checked_add_signed(rhs.y).unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
struct Map {
    start: Position,
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn get_tile(&self, position: &Position) -> Option<&Tile> {
        self.tiles
            .get(position.y)
            .and_then(|line| line.get(position.x))
    }

    fn follow(&self, path: &[Position], pos: &Position) -> Option<Position> {
        self.get_tile(pos).and_then(|tile| {
            tile.connected()
                .into_iter()
                .map(|rel_pos| pos + &rel_pos)
                .filter(|next_pos| !path.contains(next_pos))
                .flat_map(|next_pos| self.get_tile(&next_pos).map(|tile| (next_pos, tile)))
                .find(|(next_pos, tile)| {
                    // Only new positions that connects back to pos
                    tile.connected()
                        .into_iter()
                        .any(|rel_pos| next_pos + &rel_pos == *pos)
                })
                .map(|(next_pos, _)| next_pos)
        })
    }

    fn cycle(&self) -> Vec<Position> {
        let mut path = vec![self.start.clone()];
        while let Some(next) = path.last().and_then(|pos| self.follow(&path, pos)) {
            path.push(next);
        }
        path
    }
}

fn main() {
    let input: Vec<_> = include_str!("./input")
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect();

    let tiles: Vec<Vec<_>> = input
        .into_iter()
        .map(|line| line.chars().map(Tile::char_to_tile).collect())
        .collect();

    let start = tiles
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, tile)| {
                if let Tile::Start = tile {
                    Some(Position { x, y })
                } else {
                    None
                }
            })
        })
        .unwrap();

    let map = Map { start, tiles };

    // Part one
    let cycle = map.cycle();
    let max_distance = cycle.len() / 2;
    println!("{max_distance}");

    // Part two
    // Close the cycle
    let mut cycle = cycle;
    cycle.push(cycle.first().unwrap().clone());

    let xs = cycle.iter().map(|pos| pos.x as isize);
    let ys = cycle.iter().map(|pos| pos.y as isize);

    // Shoelace
    let area = xs
        .clone()
        .zip(ys.clone().skip(1))
        .zip(xs.skip(1))
        .zip(ys)
        .map(|(((x1, y2), x2), y1)| (x1 * y2) - (x2 * y1))
        .sum::<isize>()
        .abs()
        / 2;

    // Modified Pick's theoerem
    let tiles_inside = area - (cycle.len() as isize / 2) + 1;
    println!("{tiles_inside}");
}
