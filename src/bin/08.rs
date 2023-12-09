use std::collections::HashMap;

advent_of_code::solution!(8);

pub struct NodeMap {
    pub nodes: HashMap<String, Vec<String>>,
}

pub fn parse(input: &str) -> (Vec<usize>, NodeMap) {
    let instruction_mapping: HashMap<char, usize> = HashMap::from([('L', 0), ('R', 1)]);
    let mut lines = input.lines().filter(|line| !line.is_empty());
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|instruction| instruction_mapping.get(&instruction).unwrap().to_owned())
        .collect();
    let nodes = lines
        .map(|line| {
            let mut line_split = line.split(" = ");
            let node_name = line_split.next().unwrap();
            let node_value: Vec<String> = line_split
                .next()
                .unwrap()
                .replace(['(', ')'], "")
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            (node_name.to_string(), node_value.to_vec())
        })
        .collect();
    (instructions, NodeMap { nodes })
}

pub fn get_step_count(
    mut current_node_name: String,
    instructions: &Vec<usize>,
    node_map: &NodeMap,
) -> u64 {
    let mut instruction_index = 0;
    let mut instruction_count = 0;
    let instructions_length = instructions.len();

    while !current_node_name.ends_with('Z') {
        if instruction_index == instructions_length {
            instruction_index = 0;
        }
        let current_node = node_map.nodes.get(&current_node_name).unwrap();
        current_node_name = current_node[instructions[instruction_index]].clone();
        instruction_count += 1;
        instruction_index += 1;
    }

    instruction_count
}

// created from https://www.calculatorsoup.com/calculators/math/lcm.php
pub fn least_common_multiple(a: u64, b: u64) -> u64 {
    (a * b) / greatest_common_divisor(a, b)
}

pub fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    while b > 0 {
        let tmp_a = a;
        a = b;
        b = tmp_a % b;
    }

    a
}

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, node_map) = parse(input);

    let nodes_to_process = node_map.nodes.keys().filter(|key| key.ends_with('A'));
    let step_count: Vec<u64> = nodes_to_process
        .map(|node_name| get_step_count(node_name.to_string(), &instructions, &node_map))
        .collect();

    Some(step_count[0])
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, node_map) = parse(input);

    let nodes_to_process = node_map.nodes.keys().filter(|key| key.ends_with('A'));
    let step_count = nodes_to_process
        .map(|node_name| get_step_count(node_name.to_string(), &instructions, &node_map));

    let lcm = step_count.fold(1, least_common_multiple);

    Some(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
