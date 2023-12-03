advent_of_code::solution!(3);

pub enum MapObject {
    PartNumber(PartNumber),
    Symbol(Symbol),
    Coordinate(Coordinate),
}

pub struct PartNumber {
    pub number: u32,
    pub locations: Vec<Coordinate>,
}

pub struct Symbol {
    pub location: Coordinate,
    pub symbol: char,
}

pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

impl PartNumber {
    pub fn is_valid(&self, symbol: &Symbol) -> bool {
        let col_indices = self
            .locations
            .iter()
            .map(|location| location.col)
            .collect::<Vec<_>>();
        let row_index = self.locations[0].row;

        let symbol_location = &symbol.location;

        let same_row_valid =
            row_index == symbol_location.row && check_for_symbol(symbol_location, &col_indices);
        let upper_row_valid = row_index == (symbol_location.row - 1)
            && check_for_symbol(symbol_location, &col_indices);
        let lower_row_valid = row_index == (symbol_location.row + 1)
            && check_for_symbol(symbol_location, &col_indices);

        upper_row_valid || same_row_valid || lower_row_valid
    }
}

pub fn check_for_symbol(symbol_location: &Coordinate, col_indices: &[i32]) -> bool {
    if col_indices.contains(&(symbol_location.col - 1))
        || col_indices.contains(&(symbol_location.col))
        || col_indices.contains(&(symbol_location.col + 1))
    {
        return true;
    }

    false
}

pub fn parse(input: &str) -> Vec<Vec<MapObject>> {
    let mut engine_parts: Vec<Vec<MapObject>> = vec![];
    input
        .split('\n')
        .filter(|schematic_line| !schematic_line.is_empty())
        .enumerate()
        .for_each(|(line_index, schematic_line)| {
            let mut parts = schematic_line.chars().enumerate().peekable();

            let mut converted_parts: Vec<MapObject> = vec![];
            let mut full_part_number = String::new();
            let mut locations: Vec<Coordinate> = vec![];

            while let Some((index, part)) = parts.next() {
                if part.is_numeric() {
                    full_part_number.push(part);
                    locations.push(Coordinate {
                        row: line_index as i32,
                        col: index as i32,
                    });
                    if parts.peek().is_none() {
                        converted_parts.push(MapObject::PartNumber(PartNumber {
                            number: full_part_number.parse::<u32>().unwrap(),
                            locations,
                        }));
                        full_part_number = String::new();
                        locations = vec![];
                    }
                } else {
                    if !full_part_number.is_empty() {
                        converted_parts.push(MapObject::PartNumber(PartNumber {
                            number: full_part_number.parse::<u32>().unwrap(),
                            locations,
                        }));
                        full_part_number = String::new();
                        locations = vec![];
                    }
                    if part != '.' {
                        converted_parts.push(MapObject::Symbol(Symbol {
                            location: Coordinate {
                                row: line_index as i32,
                                col: index as i32,
                            },
                            symbol: part,
                        }))
                    }
                }
            }
            engine_parts.push(converted_parts);
        });

    engine_parts
}

pub fn part_one(input: &str) -> Option<u32> {
    let engine_parts = parse(input);
    let part_numbers: Vec<&PartNumber> = engine_parts
        .iter()
        .flat_map(|engine_part| {
            engine_part
                .iter()
                .filter_map(|map_object| match map_object {
                    MapObject::PartNumber(part_number) => Some(part_number),
                    _ => None,
                })
        })
        .collect();
    let symbols: Vec<&Symbol> = engine_parts
        .iter()
        .flat_map(|engine_part| {
            engine_part
                .iter()
                .filter_map(|map_object| match map_object {
                    MapObject::Symbol(symbol) => Some(symbol),
                    _ => None,
                })
        })
        .collect();

    let sum = part_numbers.into_iter().fold(0, |acc, part_number| {
        let mut total = 0;
        symbols.iter().for_each(|symbol| {
            if part_number.is_valid(symbol) {
                total += part_number.number;
            }
        });
        acc + total
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let engine_parts = parse(input);
    let part_numbers: Vec<&PartNumber> = engine_parts
        .iter()
        .flat_map(|engine_part| {
            engine_part
                .iter()
                .filter_map(|map_object| match map_object {
                    MapObject::PartNumber(part_number) => Some(part_number),
                    _ => None,
                })
        })
        .collect();
    let symbols: Vec<&Symbol> = engine_parts
        .iter()
        .flat_map(|engine_part| {
            engine_part
                .iter()
                .filter_map(|map_object| match map_object {
                    MapObject::Symbol(symbol) => {
                        if symbol.symbol == '*' {
                            Some(symbol)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
        })
        .collect();

    let total = symbols.iter().fold(0, |acc, symbol| {
        let valid_parts: Vec<u32> = part_numbers
            .iter()
            .filter_map(|part_number| {
                if part_number.is_valid(symbol) {
                    Some(part_number.number)
                } else {
                    None
                }
            })
            .collect();

        if valid_parts.len() == 2 {
            acc + valid_parts.iter().product::<u32>()
        } else {
            acc
        }
    });

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(413));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6756));
    }
}
