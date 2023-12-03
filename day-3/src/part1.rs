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
}

#[derive(Debug, Clone)]
struct Row {
    part_numbers: Vec<PartNumber>,
    items: Vec<Cell>,
}

impl Row {
    fn new(input: &str) -> Self {
        let items: Vec<Cell> = input.chars().map(Cell).collect();
        let mut part_numbers: Vec<PartNumber> = Vec::new();

        let mut start: Option<usize> = None;
        for (i, cell) in items.iter().enumerate() {
            if cell.is_empty() {
                if start.is_some() {
                    part_numbers.push(PartNumber {
                        start: start.unwrap() as u32,
                        end: (i - 1) as u32,
                    });
                    start = None;
                }
                continue;
            }

            if start.is_some() && cell.is_identifier() {
                part_numbers.push(PartNumber {
                    start: start.unwrap() as u32,
                    end: (i - 1) as u32,
                });
                start = None;
            }

            if start.is_none() && !cell.is_identifier() {
                start = Some(i);
            }

            match start {
                Some(x) if i == items.len() - 1 => {
                    part_numbers.push(PartNumber {
                        start: x as u32,
                        end: i as u32,
                    });
                }
                _ => {}
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

pub fn solution(input: &str) -> u32 {
    let grid = Grid::new(input);
    let mut total = 0;

    for (i, row) in grid.items.iter().enumerate() {
        for part_number in &row.part_numbers {
            use std::cmp::min;

            let start_h = if i == 0 { i } else { i - 1 };
            let check_height = start_h..min(i + 2, grid.height());

            let start_w = if part_number.start == 0 {
                0
            } else {
                part_number.start - 1
            };
            let check_width = start_w..min(part_number.end + 2, grid.width() as u32);

            let mut is_valid = false;
            for x in check_width {
                for y in check_height.clone() {
                    let cell = grid.get(x as usize, y);
                    is_valid |= cell.is_some_and(|cell| cell.is_identifier())
                }
            }

            if is_valid {
                let mut result = String::new();
                for x in part_number.start..part_number.end + 1 {
                    let thing = grid.get(x as usize, i).expect("cell should be something");
                    result.push(thing.0)
                }
                let number = result.parse::<u32>().expect("parsing error");
                total += number;
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
        let data = include_str!("../data/test-1.txt");
        let result = solution(data);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_diagonal() {
        let data = include_str!("../data/test-2.txt");
        let result = solution(data);

        assert_eq!(result, 200)
    }
}
