use std::collections::HashSet;

advent_of_code::solution!(11);

pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn get_manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    let space_image = input
        .lines()
        .filter(|space_point| !space_point.is_empty())
        .map(|space_point| space_point.chars().collect())
        .collect();

    space_image
}

pub fn get_space_map_info(space_image: Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>, Vec<Point>) {
    let mut row_expandor_indices: Vec<usize> = Vec::new();
    let mut non_column_expandor_indices: HashSet<usize> = HashSet::new();
    let total_columns = 0..space_image[0].len();
    let mut points: Vec<Point> = Vec::new();

    space_image.iter().enumerate().for_each(|(y_index, row)| {
        if row.iter().all(|space_point| space_point == &'.') {
            row_expandor_indices.push(y_index);
        }
        let indices: Vec<usize> = row
            .iter()
            .enumerate()
            .filter_map(|(x_index, &space_point)| {
                if space_point == '#' {
                    points.push(Point {
                        x: x_index as i64,
                        y: y_index as i64,
                    });
                    Some(x_index)
                } else {
                    None
                }
            })
            .collect();
        non_column_expandor_indices.extend(indices);
    });

    let column_expandor_indices = total_columns
        .filter(|column_index| !non_column_expandor_indices.contains(column_index))
        .collect::<Vec<usize>>();

    (row_expandor_indices, column_expandor_indices, points)
}

pub fn part_one(input: &str) -> Option<i64> {
    let space_image = parse(input);
    let (row_expandor_indices, column_expandor_indices, mut points) =
        get_space_map_info(space_image);

    points.iter_mut().for_each(|point| {
        let prefix_columns = column_expandor_indices
            .iter()
            .filter(|column_index| **column_index < point.x as usize)
            .count();
        let prefix_rows = row_expandor_indices
            .iter()
            .filter(|row_index| **row_index < point.y as usize)
            .count();
        point.x += prefix_columns as i64;
        point.y += prefix_rows as i64;
    });

    let mut distances: Vec<i64> = Vec::new();

    for point_index in 0..points.len() - 1 {
        for other_point_index in point_index + 1..points.len() {
            let distance = points[point_index].get_manhattan_distance(&points[other_point_index]);
            distances.push(distance);
        }
    }

    Some(distances.iter().sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    let space_image = parse(input);
    let (row_expandor_indices, column_expandor_indices, mut points) =
        get_space_map_info(space_image);

    points.iter_mut().for_each(|point| {
        let prefix_columns = column_expandor_indices
            .iter()
            .filter(|column_index| **column_index < point.x as usize)
            .count();
        let prefix_rows = row_expandor_indices
            .iter()
            .filter(|row_index| **row_index < point.y as usize)
            .count();
        point.x += (prefix_columns * (1_000_000 - 1)) as i64;
        point.y += (prefix_rows * (1_000_000 - 1)) as i64;
    });

    let mut distances: Vec<i64> = Vec::new();

    for point_index in 0..points.len() - 1 {
        for other_point_index in point_index + 1..points.len() {
            let distance = points[point_index].get_manhattan_distance(&points[other_point_index]);
            distances.push(distance);
        }
    }

    Some(distances.iter().sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
