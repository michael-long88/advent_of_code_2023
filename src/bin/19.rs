advent_of_code::solution!(19);

pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

pub struct Workflow {
    pub name: String,
    pub conditions: Vec<Condition>,
}

pub enum Condition {
    If(Check),
    Else(String),
}

pub struct Check {
    pub subpart: String,
    pub operator: Operator,
    pub comparison: usize,
    pub result: String,
}

pub enum Operator {
    GreaterThan,
    LessThan,
}

pub fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let mut splits = input.trim().split("\n\n");

    let workflows = splits
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut name_conditions_split = line.split('{');
            let name = name_conditions_split.next().unwrap();
            let binding = name_conditions_split.next().unwrap().replace('}', "");
            let rules = binding.split(',');
            let conditions = rules
                .map(|rule| {
                    if rule.contains('<') {
                        let mut first_split = rule.split('<');
                        let subpart = first_split.next().unwrap().to_owned();
                        let mut second_split = first_split.next().unwrap().split(':');
                        let comparison = second_split.next().unwrap().parse::<usize>().unwrap();
                        let result = second_split.next().unwrap().to_owned();

                        Condition::If(Check {
                            subpart,
                            operator: Operator::LessThan,
                            comparison,
                            result,
                        })
                    } else if rule.contains('>') {
                        let mut first_split = rule.split('>');
                        let subpart = first_split.next().unwrap().to_owned();
                        let mut second_split = first_split.next().unwrap().split(':');
                        let comparison = second_split.next().unwrap().parse::<usize>().unwrap();
                        let result = second_split.next().unwrap().to_owned();

                        Condition::If(Check {
                            subpart,
                            operator: Operator::GreaterThan,
                            comparison,
                            result,
                        })
                    } else {
                        Condition::Else(rule.to_owned())
                    }
                })
                .collect();
            Workflow {
                name: name.to_owned(),
                conditions,
            }
        })
        .collect();

    let parts = splits
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let binding = line.replace(['{', '}'], "");
            let mut split = binding.split(',');
            // println!("{:?}", split.clone().collect::<String>());
            let x = split
                .next()
                .unwrap()
                .replace("x=", "")
                .parse::<usize>()
                .unwrap();
            let m = split
                .next()
                .unwrap()
                .replace("m=", "")
                .parse::<usize>()
                .unwrap();
            let a = split
                .next()
                .unwrap()
                .replace("a=", "")
                .parse::<usize>()
                .unwrap();
            let s = split
                .next()
                .unwrap()
                .replace("s=", "")
                .parse::<usize>()
                .unwrap();

            Part { x, m, a, s }
        })
        .collect();

    (workflows, parts)
}

pub fn get_result(workflows: &Vec<Workflow>, part: &Part, start_index: usize) -> String {
    let start_workflow = &workflows[start_index];
    let mut result = start_workflow.name.to_owned();

    if result != "A" && result != "R" {
        for condition in &start_workflow.conditions {
            match condition {
                Condition::If(check) => {
                    if check_condition(check, part) {
                        result = check.result.to_owned();
                        break;
                    }
                }
                Condition::Else(rule) => {
                    result = rule.to_owned();
                    break;
                }
            }
        }
        if result != "A" && result != "R" {
            let start_index = workflows
                .iter()
                .position(|workflow| workflow.name == result)
                .unwrap();
            return get_result(workflows, part, start_index);
        }
    }

    result
}

pub fn check_condition(check: &Check, part: &Part) -> bool {
    let subpart = match check.subpart.as_str() {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => panic!("Unknown subpart"),
    };

    match check.operator {
        Operator::GreaterThan => subpart > check.comparison,
        Operator::LessThan => subpart < check.comparison,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (workflows, parts) = parse(input);
    let start_index = workflows
        .iter()
        .position(|workflow| workflow.name == "in")
        .unwrap();

    let total: usize = parts
        .iter()
        .filter(|part| get_result(&workflows, part, start_index) == "A")
        .map(|part| part.x + part.m + part.a + part.s)
        .sum();

    Some(total)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19_114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
