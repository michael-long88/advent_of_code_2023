use std::ops::Range;

advent_of_code::solution!(5);

pub struct AlmanacMap {
    pub source_ranges: Vec<Range<u64>>,
    pub destination_ranges: Vec<Range<u64>>,
}

pub struct Seeds {
    pub seed_numbers: Vec<u64>,
}

pub fn parse(input: &str) -> (Seeds, Vec<AlmanacMap>) {
    let mut seed_numbers: Vec<u64> = Vec::new();

    let maps = input
        .split("\n\n")
        .filter(|map| !map.is_empty())
        .enumerate()
        .map(|(index, map)| {
            if index == 0 {
                seed_numbers = map
                    .replace("seeds: ", "")
                    .split_whitespace()
                    .map(|number| number.parse::<u64>().unwrap())
                    .collect();
            }
            let mut source_ranges: Vec<Range<u64>> = Vec::new();
            let mut destination_ranges: Vec<Range<u64>> = Vec::new();
            let mut map_split = map.split('\n');
            map_split.next();
            map_split
                .filter(|number_sequence| !number_sequence.is_empty())
                .for_each(|number_sequence| {
                    let numbers = number_sequence
                        .split_whitespace()
                        .map(|number| number.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();

                    source_ranges.push(numbers[1]..(numbers[1] + numbers[2]));
                    destination_ranges.push(numbers[0]..(numbers[0] + numbers[2]));
                });

            source_ranges.reverse();
            destination_ranges.reverse();

            AlmanacMap {
                source_ranges,
                destination_ranges,
            }
        })
        .collect();

    (Seeds { seed_numbers }, maps)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    let locations: Vec<u64> = seeds
        .seed_numbers
        .iter()
        .map(|seed| {
            let mut location = *seed;
            for map in &maps {
                for (index, source_range) in map.source_ranges.iter().enumerate() {
                    if source_range.contains(&location) {
                        let destination_range = &map.destination_ranges[index];
                        let offset = location - source_range.start;
                        location = destination_range.start + offset;

                        break;
                    }
                }
            }

            location
        })
        .collect();

    Some(*locations.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, maps) = parse(input);
    let mut seed_ranges: Vec<Range<u64>> = seeds
        .seed_numbers
        .chunks_exact(2)
        .map(|seed_pair| {
            let start = seed_pair[0];
            let end = seed_pair[0] + seed_pair[1];

            start..end
        })
        .collect();

    // I wish I could take credit for this, but this was all thanks to this
    // repo: https://github.com/Tyranties/AOC-2023/blob/main/day_05/day_05_part_2.py
    maps.iter().for_each(|map| {
        let mut new_seeds = Vec::new();

        while let Some(seed_range) = seed_ranges.pop() {
            let mut overlap_start_less_than_end = false;
            for (index, source_range) in map.source_ranges.iter().enumerate() {
                let overlap_start = seed_range.start.max(source_range.start);
                let overlap_end = seed_range.end.min(source_range.end);

                if overlap_start < overlap_end {
                    new_seeds.push(
                        (overlap_start - source_range.start + map.destination_ranges[index].start)
                            ..(overlap_end - source_range.start
                                + map.destination_ranges[index].start),
                    );
                    if overlap_start > seed_range.start {
                        seed_ranges.push(seed_range.start..overlap_start);
                    }
                    if overlap_end < seed_range.end {
                        seed_ranges.push(overlap_end..seed_range.end);
                    }
                    overlap_start_less_than_end = true;
                    break;
                }
            }
            if !overlap_start_less_than_end {
                new_seeds.push(seed_range.clone());
            }
        }
        seed_ranges = new_seeds;
    });

    let min_seed = seed_ranges
        .iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap();

    Some(min_seed.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
