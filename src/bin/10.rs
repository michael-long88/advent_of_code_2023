use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq)]
pub enum PipeType {
    Vertical,
    Horizontal,
    NE90,
    NW90,
    SE90,
    SW90,
    Ground,
    Start,
}

pub struct Pipes {
    pub pipes: Vec<Vec<Pipe>>,
}

pub struct Pipe {
    pub pipe_type: PipeType,
    pub location: (i32, i32),
}

impl Pipe {
    pub fn get_next_pipe(&self, previous_location: (i32, i32), pipes: &Pipes) -> Pipe {
        let (x, y) = self.location;
        let (previous_x, previous_y) = previous_location;
        let mut next_location = (x, y);

        match self.pipe_type {
            PipeType::Vertical => {
                if previous_y < y {
                    next_location = (x, y + 1);
                } else {
                    next_location = (x, y - 1);
                }
            }
            PipeType::Horizontal => {
                if previous_x < x {
                    next_location = (x + 1, y);
                } else {
                    next_location = (x - 1, y);
                }
            }
            PipeType::NE90 => {
                if previous_x > x {
                    next_location = (x, y - 1);
                } else {
                    next_location = (x + 1, y);
                }
            }
            PipeType::NW90 => {
                if previous_x < x {
                    next_location = (x, y - 1);
                } else {
                    next_location = (x - 1, y);
                }
            }
            PipeType::SE90 => {
                if previous_x > x {
                    next_location = (x, y + 1);
                } else {
                    next_location = (x + 1, y);
                }
            }
            PipeType::SW90 => {
                if previous_x < x {
                    next_location = (x, y + 1);
                } else {
                    next_location = (x - 1, y);
                }
            }
            PipeType::Start => {
                // very first check
                if self.location == previous_location {
                    if self.location.1 != 0 {
                        let next_possible_pipe = &pipes.pipes[y as usize - 1][x as usize];
                        // next_location = (x, y - 1);
                        match next_possible_pipe.pipe_type {
                            PipeType::Vertical => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::SE90 => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::SW90 => {
                                return next_possible_pipe.clone();
                            }
                            _ => {}
                        }
                    }
                    if self.location.1 != pipes.pipes.len() as i32 - 1 {
                        let next_possible_pipe = &pipes.pipes[y as usize + 1][x as usize];
                        match next_possible_pipe.pipe_type {
                            PipeType::Vertical => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::NE90 => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::NW90 => {
                                return next_possible_pipe.clone();
                            }
                            _ => {}
                        }
                    }
                    if self.location.0 != 0 {
                        let next_possible_pipe = &pipes.pipes[y as usize][x as usize - 1];
                        match next_possible_pipe.pipe_type {
                            PipeType::Horizontal => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::NE90 => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::SE90 => {
                                return next_possible_pipe.clone();
                            }
                            _ => {}
                        }
                    }
                    if self.location.0 != pipes.pipes[0].len() as i32 - 1 {
                        let next_possible_pipe = &pipes.pipes[y as usize][x as usize + 1];
                        match next_possible_pipe.pipe_type {
                            PipeType::Horizontal => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::NW90 => {
                                return next_possible_pipe.clone();
                            }
                            PipeType::SW90 => {
                                return next_possible_pipe.clone();
                            }
                            _ => {}
                        }
                    }
                } else {
                    return self.clone();
                }
            }
            _ => panic!("It shouldn't be possible to traverse to normal ground"),
        }

        if previous_location == (0, 4) {
            println!("{:?}", next_location);
        }

        pipes.pipes[next_location.1 as usize][next_location.0 as usize].clone()
    }
}

impl Clone for Pipe {
    fn clone(&self) -> Self {
        Pipe {
            pipe_type: self.pipe_type.clone(),
            location: self.location,
        }
    }
}

impl Clone for PipeType {
    fn clone(&self) -> Self {
        match self {
            PipeType::Vertical => PipeType::Vertical,
            PipeType::Horizontal => PipeType::Horizontal,
            PipeType::NE90 => PipeType::NE90,
            PipeType::NW90 => PipeType::NW90,
            PipeType::SE90 => PipeType::SE90,
            PipeType::SW90 => PipeType::SW90,
            PipeType::Ground => PipeType::Ground,
            PipeType::Start => PipeType::Start,
        }
    }
}

pub fn parse(input: &str) -> (Pipe, Pipes) {
    let mut starting_pipe = Pipe {
        pipe_type: PipeType::Start,
        location: (0, 0),
    };
    let pipes = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .map(|(pipe_index, pipe)| {
                    let pipe_type = match pipe {
                        '|' => PipeType::Vertical,
                        '-' => PipeType::Horizontal,
                        'L' => PipeType::NE90,
                        'J' => PipeType::NW90,
                        'F' => PipeType::SE90,
                        '7' => PipeType::SW90,
                        '.' => PipeType::Ground,
                        'S' => PipeType::Start,
                        _ => panic!("Invalid pipe type"),
                    };

                    if pipe == 'S' {
                        starting_pipe = Pipe {
                            pipe_type: pipe_type.clone(),
                            location: (pipe_index as i32, line_index as i32),
                        };
                    }

                    Pipe {
                        pipe_type,
                        location: (pipe_index as i32, line_index as i32),
                    }
                })
                .collect()
        })
        .collect();

    (starting_pipe, Pipes { pipes })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (starting_pipe, pipes) = parse(input);
    let mut pipe_route: Vec<Pipe> = Vec::new();

    let mut previous_pipe = starting_pipe.clone();
    let mut previous_location = previous_pipe.location;
    let mut current_pipe = previous_pipe.get_next_pipe(previous_location, &pipes);
    pipe_route.push(current_pipe.clone());

    while current_pipe.pipe_type != PipeType::Start {
        // println!("Current Pipe: {:?} at {:?}", current_pipe.pipe_type, current_pipe.location);
        previous_pipe = current_pipe.clone();
        // println!("Previous Location: {:?}", previous_location);
        current_pipe = current_pipe.get_next_pipe(previous_location, &pipes);
        // println!("Next Pipe: {:?} at {:?}", current_pipe.pipe_type, current_pipe.location);
        previous_location = previous_pipe.location;
        pipe_route.push(current_pipe.clone());
    }

    Some(pipe_route.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (starting_pipe, pipes) = parse(input);
    let mut pipe_route: Vec<Pipe> = Vec::new();

    let mut previous_pipe = starting_pipe.clone();
    let mut previous_location = previous_pipe.location;
    let mut current_pipe = previous_pipe.get_next_pipe(previous_location, &pipes);
    pipe_route.push(current_pipe.clone());

    while current_pipe.pipe_type != PipeType::Start {
        // println!("Current Pipe: {:?} at {:?}", current_pipe.pipe_type, current_pipe.location);
        previous_pipe = current_pipe.clone();
        // println!("Previous Location: {:?}", previous_location);
        current_pipe = current_pipe.get_next_pipe(previous_location, &pipes);
        // println!("Next Pipe: {:?} at {:?}", current_pipe.pipe_type, current_pipe.location);
        previous_location = previous_pipe.location;
        pipe_route.push(current_pipe.clone());
    }

    let valid_pipe_locations: HashSet<(i32, i32)> =
        HashSet::from_iter(pipe_route.iter().map(|pipe| pipe.location));
    let valid_pipe_count_locations: HashSet<(i32, i32)> = HashSet::from_iter(
        pipe_route
            .iter()
            .filter(|pipe| {
                pipe.pipe_type == PipeType::Vertical
                    || pipe.pipe_type == PipeType::SE90
                    || pipe.pipe_type == PipeType::SW90
            })
            .map(|pipe| pipe.location),
    );

    let mut internal_pipe_count = 0;
    for pipe_row in pipes.pipes {
        let mut row_count = 0;
        for pipe in pipe_row {
            if valid_pipe_count_locations.contains(&pipe.location) {
                row_count ^= 1;
            }
            if !valid_pipe_locations.contains(&pipe.location) && row_count == 1 {
                internal_pipe_count += 1;
            }
        }
    }

    Some(internal_pipe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
