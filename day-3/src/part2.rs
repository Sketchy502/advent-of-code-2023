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
    start: usize,
    end: usize,
    value: usize,
}

impl PartNumber {
    fn contains(&self, x: usize) -> bool {
        x >= self.start && x <= self.end
    }
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
                if let Some(s) = start {
                    part_numbers.push(PartNumber {
                        start: start.unwrap(),
                        end: i - 1,
                        value: {
                            let mut v = String::new();

                            for x in s..i {
                                let thing = items.get(x).expect("cell should be something");
                                v.push(thing.0)
                            }

                            v.parse::<usize>().expect("parsing error")
                        },
                    });
                    start = None;
                }
                continue;
            }

            match start {
                Some(s) if i == items.len() - 1 => {
                    part_numbers.push(PartNumber {
                        start: s,
                        end: i,
                        value: {
                            let mut v = String::new();
                            for x in s..(i + 1) {
                                let thing = items.get(x).expect("cell should be something");
                                v.push(thing.0)
                            }
                            v.parse::<usize>().expect("parsing error")
                        },
                    });
                }
                Some(s) if cell.is_identifier() => {
                    part_numbers.push(PartNumber {
                        start: s,
                        end: i - 1,
                        value: {
                            let mut v = String::new();

                            for x in s..i {
                                let thing = items.get(x).expect("cell should be something");
                                v.push(thing.0)
                            }

                            v.parse::<usize>().expect("parsing error")
                        },
                    });
                    start = None;
                }
                None if !cell.is_identifier() => {
                    start = Some(i);
                }
                _ => {}
            }
        }

        Self {
            items,
            part_numbers,
        }
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

    fn height(&self) -> usize {
        self.items.len()
    }
}

pub fn solution(input: &str) -> usize {
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
                    part_number.contains(j)
                        || part_number.contains(if j == 0 { 0 } else { j - 1 })
                        || part_number.contains(j + 1)
                })
                .collect::<Vec<_>>();

            if search_space_parts.len() == 2 {
                total += search_space_parts[0].value * search_space_parts[1].value;
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
        let data = include_str!("../data/test-3.txt");
        let result = solution(data);

        assert_eq!(result, 467835);
    }

    #[test]
    fn test_more() {
        let data = include_str!("../data/test-4.txt");
        let result = solution(data);
        assert_eq!(result, 6756);
    }
}
