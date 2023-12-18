use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn parse(input: &str) -> Vec<Vec<u64>> {
    let initialization_sequence = input
        .trim()
        .split(',')
        .map(|step| {
            step.as_bytes()
                .iter()
                .map(|byte| u64::from(*byte))
                .collect::<Vec<u64>>()
        })
        .collect();

    initialization_sequence
}

pub fn caclulate(current_value: u64, next_value: u64) -> u64 {
    let mut new_value = current_value + next_value;
    new_value *= 17;

    new_value % 256
}

pub fn part_one(input: &str) -> Option<u64> {
    let initialization_sequence = parse(input);
    let sequence_totals = initialization_sequence.iter().map(|sequence| {
        let mut total = 0;
        sequence.iter().for_each(|step| {
            total = caclulate(total, *step);
        });
        total
    });

    Some(sequence_totals.sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    // (sequence, box number): lens value
    let mut lenses: HashMap<(Vec<u64>, u64), u64> = HashMap::new();
    // box number: lenses
    let mut boxes: HashMap<u64, Vec<Vec<u64>>> = HashMap::new();
    let initialization_sequence = parse(input);
    let equal_sign_ascii_value: u64 = 61;
    let minus_sign_ascii_value: u64 = 45;
    initialization_sequence.iter().for_each(|sequence| {
        let mut box_number = 0;

        let index = sequence
            .iter()
            .position(|&step| step == equal_sign_ascii_value || step == minus_sign_ascii_value)
            .unwrap();

        sequence[..index].iter().for_each(|step| {
            box_number = caclulate(box_number, *step);
        });

        if sequence[index] == equal_sign_ascii_value {
            let values = sequence[..index].to_vec();
            let lens = sequence[index + 1] - 48;
            lenses
                .entry((values.clone(), box_number))
                .and_modify(|e| *e = lens)
                .or_insert(lens);

            if boxes.get(&box_number).is_none() {
                boxes.entry(box_number).or_default().push(values.clone());
            } else if !boxes.get(&box_number).unwrap().contains(&values) {
                boxes.get_mut(&box_number).unwrap().push(values.clone());
            }
        } else if sequence[index] == minus_sign_ascii_value {
            let values = sequence[..index].to_vec();
            lenses.remove(&(values.clone(), box_number));
            if let Some(lens_vec) = boxes.get_mut(&box_number) {
                if let Some(index_to_remove) = lens_vec.iter().position(|x| *x == values) {
                    lens_vec.remove(index_to_remove);
                }
            }
        }
    });

    let totals = boxes.keys().map(|box_number| {
        let mut total = 0;
        boxes
            .get(box_number)
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(index, sequence)| {
                let focal_strength = lenses.get(&(sequence.to_vec(), *box_number)).unwrap();

                let lens_total = (box_number + 1) * (index as u64 + 1) * focal_strength;

                total += lens_total;
            });

        total
    });

    Some(totals.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
