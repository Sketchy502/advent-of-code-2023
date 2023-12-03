#![allow(unused)]

fn main() {
    let input = include_str!("./input/input-1.txt");
    let output = part1(input);

    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Cell(char);

impl Cell {
    fn is_empty(&self) -> bool {
        self.0 == '.'
    }

    fn is_identifier(&self) -> bool {
        !self.is_empty() && !self.0.is_numeric()
    }
}

#[derive(Debug, Copy, Clone)]
struct PartNumber {
    start: u32,
    end: u32,
    value: usize,
}

impl PartNumber {
    fn contains(&self, x: u32) -> bool {
        x >= self.start && x <= self.end
    }
}

#[derive(Debug, Clone)]
struct Row {
    part_numbers: Vec<PartNumber>,
    items: Vec<Cell>,
}

fn parse_part_number(part_number: &PartNumber, grid: &Grid) -> u32 {
    let mut result = String::new();
    for x in part_number.start..part_number.end + 1 {
        let thing = grid.get(x as usize, 0).expect("cell should be something");
        result.push(thing.0)
    }
    result.parse::<u32>().expect("parsing error")
}

impl Row {
    fn new(input: &str) -> Self {
        let items: Vec<Cell> = input.chars().map(Cell).collect();
        let mut part_numbers: Vec<PartNumber> = Vec::new();

        let mut start: Option<usize> = None;
        for (i, cell) in items.iter().enumerate() {
            if cell.is_empty() {
                if start.is_some() {
                    let s = start.unwrap() as u32;
                    part_numbers.push(PartNumber {
                        start: start.unwrap() as u32,
                        end: (i - 1) as u32,
                        value: {
                            let mut v = String::new();

                            for x in s..(i as u32) {
                                let thing =
                                    items.get(x as usize).expect("cell should be something");
                                v.push(thing.0)
                            }

                            v.parse::<usize>().expect("parsing error")
                        },
                    });
                    start = None;
                }
                continue;
            }

            if start.is_some() && cell.is_identifier() {
                let s = start.unwrap() as u32;
                part_numbers.push(PartNumber {
                    start: s,
                    end: (i - 1) as u32,
                    value: {
                        let mut v = String::new();

                        for x in s..(i as u32) {
                            let thing = items.get(x as usize).expect("cell should be something");
                            v.push(thing.0)
                        }

                        v.parse::<usize>().expect("parsing error")
                    },
                });
                start = None;
            }

            if start.is_none() && !cell.is_identifier() {
                start = Some(i);
            }

            if start.is_some() && i == items.len() - 1 {
                let s = start.unwrap() as u32;
                part_numbers.push(PartNumber {
                    start: start.unwrap() as u32,
                    end: i as u32,
                    value: {
                        let mut v = String::new();

                        for x in s..((i + 1) as u32) {
                            let thing = items.get(x as usize).expect("cell should be something");
                            v.push(thing.0)
                        }

                        v.parse::<usize>().expect("parsing error")
                    },
                });
            }
        }

        Self {
            items,
            part_numbers,
        }
    }

    fn get(&self, x: usize) -> Option<&Cell> {
        self.items.get(x)
    }
}

#[derive(Debug)]
struct Grid {
    items: Vec<Row>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let grid = input.lines().map(Row::new).collect();
        Self { items: grid }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.items.get(y).and_then(|row| row.get(x))
    }

    fn height(&self) -> usize {
        self.items.len()
    }

    fn width(&self) -> usize {
        self.items
            .get(0)
            .expect("Grid should have one row")
            .items
            .len()
    }
}

fn part1(input: &str) -> u32 {
    let grid = Grid::new(input);
    let mut total = 0;

    for (i, row) in grid.items.iter().enumerate() {
        for (j, cell) in row.items.iter().enumerate() {
            if cell.to_owned().0 != '*' {
                continue;
            }

            let max_h = std::cmp::min(i + 2, grid.height());
            let min_h = if i == 0 { i } else { i - 1 };
            let search_space_rows = grid.items.get(min_h..max_h).unwrap();
            let search_space_parts = search_space_rows
                .iter()
                .flat_map(|row| &row.part_numbers)
                .filter(|part_number| {
                    part_number.contains(j as u32)
                        || part_number.contains(if j == 0 { 0 } else { j - 1 } as u32)
                        || part_number.contains((j + 1) as u32)
                })
                .collect::<Vec<_>>();

            if search_space_parts.len() == 2 {
                total += (search_space_parts[0].value * search_space_parts[1].value) as u32;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result() {
        let data = include_str!("./input/test-3.txt");
        let result = part1(data);

        assert_eq!(result, 467835);
    }

    #[test]
    fn test_more() {
        let data = include_str!("./input/test-4.txt");
        let result = part1(data);
        assert_eq!(result, 6756);
    }
}
