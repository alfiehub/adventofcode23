use std::collections::HashMap;

#[derive(Debug)]
enum Value {
    Number(u32),
    Symbol(char),
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    /// Return a list of all positions surrounding the given position. Does not account for upper
    /// bounds for rows or columns, ie. row and column can overflow the grid.
    fn adjacents(&self) -> Vec<Position> {
        let mut positions = vec![];
        let left = self.column.checked_sub(1);
        if let Some(above) = self.row.checked_sub(1) {
            if let Some(left) = left {
                // Up-left
                positions.push(Position {
                    row: above,
                    column: left,
                });
            }
            // Up
            positions.push(Position {
                row: above,
                column: self.column,
            });
            // Up-right
            positions.push(Position {
                row: above,
                column: self.column + 1,
            });
        };

        if let Some(left) = left {
            // Left
            positions.push(Position {
                row: self.row,
                column: left,
            });
            // Down-left
            positions.push(Position {
                row: self.row + 1,
                column: left,
            });
        }

        // Down
        positions.push(Position {
            row: self.row + 1,
            column: self.column,
        });

        // Right
        positions.push(Position {
            row: self.row,
            column: self.column + 1,
        });
        // Down-right
        positions.push(Position {
            row: self.row + 1,
            column: self.column + 1,
        });

        positions
    }
}

struct Grid {
    rows: Vec<Vec<Option<Value>>>,
}

impl Grid {
    fn get(&self, position: &Position) -> Option<&Value> {
        self.rows
            .get(position.row)
            .and_then(|row| row.get(position.column).map(|value| value.as_ref()))
            .flatten()
    }

    fn is_symbol(&self, position: &Position) -> bool {
        self.get(position)
            .map(|value| matches!(value, Value::Symbol(_)))
            .unwrap_or(false)
    }

    fn is_number(&self, position: &Position) -> bool {
        self.get(position)
            .map(|value| matches!(value, Value::Number(_)))
            .unwrap_or(false)
    }

    fn has_adjacent(&self, position: &Position, f: impl Fn(&Self, &Position) -> bool) -> bool {
        position
            .adjacents()
            .into_iter()
            .any(|adjacent| f(self, &adjacent))
    }

    fn has_adjacent_symbol(&self, position: &Position) -> bool {
        self.has_adjacent(position, Self::is_symbol)
    }

    /// Reads all symbols from left-to-right of position that is a number
    fn read_number(&self, position: &Position) -> Option<(usize, Position)> {
        // Make sure we start at a number
        if !self.is_number(position) {
            return None;
        }

        // Find the start of the number
        let start = {
            let mut start = position.column;
            if let Some(mut next) = start.checked_sub(1) {
                while let Some(Value::Number(_)) = self.get(&Position {
                    row: position.row,
                    column: next,
                }) {
                    start = next;
                    if let Some(left_next) = next.checked_sub(1) {
                        next = left_next;
                    } else {
                        break;
                    }
                }
            }
            start
        };

        // Get all the numbers
        let numbers = {
            let mut column = start;
            let mut numbers = vec![];
            while let Some(Value::Number(n)) = self.get(&Position {
                row: position.row,
                column,
            }) {
                numbers.push(n);
                column += 1;
            }
            numbers
        };

        Some((
            numbers
                .into_iter()
                .rev()
                .enumerate()
                .map(|(pow, n)| *n as usize * 10_usize.pow(pow as u32))
                .sum(),
            Position {
                row: position.row,
                column: start,
            },
        ))
    }
}

fn main() {
    let input = include_str!("./input");

    let rows: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    println!("{rows:#?}");

    let columns = rows.first().unwrap().len();

    println!("Rows: {}", rows.len());
    println!("Columns: {columns}");

    let grid = Grid {
        rows: rows
            .iter()
            .map(|row| {
                row.chars()
                    .map(|char| {
                        if char.is_numeric() {
                            char.to_digit(10).map(Value::Number)
                        } else if char == '.' {
                            None
                        } else {
                            Some(Value::Symbol(char))
                        }
                    })
                    .collect()
            })
            .collect(),
    };

    // Part one
    // Make sure to not double count numbers by storing them by start position
    let mut seen_numbers: HashMap<Position, usize> = HashMap::new();

    // Check all positions
    for row in 0..rows.len() {
        for column in 0..columns {
            let position = Position { row, column };
            if grid.has_adjacent_symbol(&position) {
                if let Some((n, start_position)) = grid.read_number(&position) {
                    if let Some(&existing_value) = seen_numbers.get(&start_position) {
                        if n != existing_value {
                            panic!(
                            "Position inserted before with different value {existing_value} != {n}"
                        );
                        }
                    } else if n > 0 {
                        seen_numbers.insert(start_position, n);
                    }
                }
            }
        }
    }
    let sum: usize = seen_numbers.values().sum();
    println!("{sum}");

    // Part two
    let mut gear_ratios: Vec<usize> = vec![];
    for row in 0..rows.len() {
        for column in 0..columns {
            let position = Position { row, column };
            if grid.is_symbol(&position) {
                // Input doesnt require you to check if the symbol is *

                let adjacent_numbers_positions: Vec<_> = position
                    .adjacents()
                    .into_iter()
                    .filter(|adjacent| grid.is_number(adjacent))
                    .collect();

                // Make sure not to double count adjacent numbers by storing them by start
                // postition
                let mut seen_numbers: HashMap<Position, usize> = HashMap::new();
                adjacent_numbers_positions.into_iter().for_each(|position| {
                    if let Some((n, start_position)) = grid.read_number(&position) {
                        seen_numbers.insert(start_position, n);
                    }
                });

                // Gear ratios are symbols with exactly two adjacent numbers
                if seen_numbers.len() == 2 {
                    gear_ratios.push(seen_numbers.values().product());
                }
            }
        }
    }
    let gear_ratios_sum: usize = gear_ratios.into_iter().sum();
    println!("{gear_ratios_sum}");
}
