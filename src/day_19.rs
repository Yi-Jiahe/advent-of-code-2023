use regex::Regex;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

type MachinePart = [usize; 4];

type Workflows<'a> = HashMap<&'a str, (Vec<WorkflowRule<'a>>, &'a str)>;

#[derive(Debug)]
struct WorkflowRule<'a>(char, char, usize, &'a str);

fn parse_input(input: &str) -> (Workflows, Vec<MachinePart>) {
    let mut input_iterator = input.split("\n").map(|line| line.trim());

    let mut workflows: Workflows = HashMap::new();

    let workflow_re = Regex::new(r"(\w+)\{(.*),(\w+)\}").unwrap();
    let rule_re = Regex::new(r"(\w+)([><])(\d+):(\w+)").unwrap();

    loop {
        let workflow = input_iterator.next().unwrap();
        if workflow == "" {
            break;
        }

        let (_, [name, rules, dump]) = workflow_re
            .captures(workflow)
            .expect("Unable to parse workflow")
            .extract();
        let mut workflow_rules = Vec::new();
        for rule in rules.split(",") {
            let (_, [rating, operator, value, destination]) = rule_re
                .captures(rule)
                .expect("Unable to parse rule")
                .extract();
            workflow_rules.push(WorkflowRule(
                rating.parse::<char>().unwrap(),
                operator.parse::<char>().unwrap(),
                value.parse::<usize>().unwrap(),
                destination,
            ));
        }
        workflows.insert(name, (workflow_rules, dump));
    }

    let mut parts = Vec::new();

    let part_re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    loop {
        if let Some(part) = input_iterator.next() {
            if part == "" {
                break;
            }
            let (_, ratings) = part_re
                .captures(part)
                .expect("Unable to parse part")
                .extract();
            // Hey it spells x-mas!
            let [x, m, a, s] = ratings.map(|r| r.parse::<usize>().unwrap());

            parts.push([x, m, a, s]);
        } else {
            break;
        }
    }

    (workflows, parts)
}

fn run_workflow<'a>(workflow: &'a (Vec<WorkflowRule>, &str), part: &MachinePart) -> &'a str {
    for WorkflowRule(rating, operator, value, destination) in &workflow.0 {
        let i = match rating {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        let matched = match operator {
            '<' => part[i] < *value,
            '>' => part[i] > *value,
            _ => unreachable!(),
        };
        if matched {
            return destination;
        }
    }
    return workflow.1;
}

fn sort_part<'a>(part: &MachinePart, workflows: &'a Workflows) -> &'a str {
    let mut workflow_name = "in";

    loop {
        let workflow = workflows.get(&workflow_name).unwrap();

        workflow_name = run_workflow(workflow, &part);

        if workflow_name == "A" || workflow_name == "R" {
            return workflow_name;
        }
    }
}

fn sort_parts<'a>(
    workflows: &Workflows,
    parts: &'a Vec<MachinePart>,
) -> (Vec<&'a MachinePart>, Vec<&'a MachinePart>) {
    let (mut accepted, mut rejected) = (Vec::new(), Vec::new());

    for part in parts {
        match sort_part(part, workflows) {
            "A" => accepted.push(part),
            "R" => rejected.push(part),
            _ => unreachable!(),
        }
    }

    (accepted, rejected)
}

pub fn day_19_sum_accepted_part_ratings(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    let (accepted, _) = sort_parts(&workflows, &parts);

    let mut acc = 0;
    for part in accepted {
        for rating in part {
            acc = acc + rating;
        }
    }

    acc
}

pub fn input_has_cycle(input: &str) -> bool {
    let (workflows, _) = parse_input(input);

    let mut simplified_workflows = HashMap::new();

    for (name, (rules, dump)) in workflows {
        let mut destinations = HashSet::from([dump]);
        for WorkflowRule(_, _, _, destination) in rules {
            destinations.insert(destination);
        }
        simplified_workflows.insert(name, destinations);
    }

    fn has_cycle(
        curr: &str,
        visited: HashSet<&str>,
        simplified_workflows: &HashMap<&str, HashSet<&str>>,
    ) -> bool {
        if curr == "A" || curr == "R" {
            return false;
        }

        if visited.contains(curr) {
            return true;
        }

        for adjacent_node in simplified_workflows.get(curr).unwrap() {
            let mut visited_new = HashSet::from([curr]);
            visited_new.extend(&visited);
            if has_cycle(adjacent_node, visited_new, simplified_workflows) {
                return true;
            }
        }

        return false;
    }

    has_cycle("in", HashSet::from([""]), &simplified_workflows)
}

fn find_all_cases<'a>(workflows: &'a Workflows) -> Vec<(Vec<&'a str>, Vec<(char, char, usize)>)> {
    fn find_cases<'a>(
        curr: &'a str,
        path: Vec<&'a str>,
        conditions: Vec<(char, char, usize)>,
        workflows: &'a Workflows,
    ) -> Vec<(Vec<&'a str>, Vec<(char, char, usize)>)> {
        let mut new_path = path.clone();
        new_path.push(curr);
        if curr == "A" || curr == "R" {
            println!("Path: {:?}, Conditions: {:?}", new_path, conditions);
            return vec![(new_path, conditions)];
        }
        let mut accepted_conditions = Vec::new();

        let workflow = workflows.get(curr).unwrap();
        for WorkflowRule(r, o, v, d) in &workflow.0 {
            let mut new_conditions = conditions.clone();
            new_conditions.push((*r, *o, *v));
            accepted_conditions.extend(find_cases(d, new_path.clone(), new_conditions, workflows));
        }
        accepted_conditions.extend(find_cases(
            workflow.1,
            new_path.clone(),
            conditions.clone(),
            workflows,
        ));

        return accepted_conditions;
    }

    find_cases("in", Vec::new(), Vec::new(), workflows)
}

fn combine_conditions(conditions: Vec<(char, char, usize)>) -> [[usize; 2]; 4] {
    let mut combined_conditions = [[0, 4001]; 4];

    for (r, o, v) in conditions {
        let i = match r {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        match o {
            '<' => combined_conditions[i][1] = v,
            '>' => combined_conditions[i][0] = v,
            _ => unreachable!(),
        };
    }

    combined_conditions
}

fn find_valid_space(conditions: [[usize; 2]; 4]) -> usize {
    let mut acc = 1;
    for i in 0..4 {
        acc = acc * ((conditions[i][1] - conditions[i][0]) - 1);
    }
    acc
}

fn find_overlaps(a: [[usize; 2]; 4], b: [[usize; 2]; 4]) -> usize {
    let mut acc = 1;

    for i in 0..4 {
        let (l, r) = (max(a[i][0], b[i][0]), min(a[i][1], b[i][1]));
        if r <= l + 1 {
            return 0;
        }
        acc = acc * ((r - l) - 1);
    }

    acc
}

pub fn day_19_number_of_combinations_of_accepted_ratings(input: &str) -> usize {
    let (workflows, _) = parse_input(input);

    let accepted_conditions: Vec<[[usize; 2]; 4]> = find_all_cases(&workflows)
        .into_iter()
        .filter(|x| *x.0.last().unwrap() == "A")
        .map(|x| combine_conditions(x.1))
        .collect();

    let n = accepted_conditions.len();

    let mut acc: isize = 0;
    for i in 0..n {
        println!("{:?}", accepted_conditions[i]);
        acc = acc + find_valid_space(accepted_conditions[i]) as isize;
        for j in (i + 1)..n {
            println!("{}, {}", i, j);
            // acc = acc - find_overlaps(accepted_conditions[i], accepted_conditions[j]) as isize;
        }
    }

    acc as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
  pv{a>1716:R,A}
  lnx{m>1548:A,A}
  rfg{s<537:gd,x>2440:R,A}
  qs{s>3448:A,lnx}
  qkq{x<1416:A,crn}
  crn{x>2662:A,R}
  in{s<1351:px,qqz}
  qqz{s>2770:qs,m<1801:hdj,R}
  gd{a>3333:R,R}
  hdj{m>838:A,pv}
  
  {x=787,m=2655,a=1222,s=2876}
  {x=1679,m=44,a=2067,s=496}
  {x=2036,m=264,a=79,s=2244}
  {x=2461,m=1339,a=466,s=291}
  {x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_day_19_sum_accepted_part_ratings() {
        assert_eq!(19114, day_19_sum_accepted_part_ratings(EXAMPLE))
    }

    #[test]
    fn test_for_cycles() {
        assert_eq!(false, input_has_cycle(EXAMPLE));
    }

    #[test]
    fn test_find_valid_space() {
        assert_eq!(
            15320205000000,
            find_valid_space([[0, 1416], [0, 4001], [0, 2006], [0, 1351]])
        );
    }

    #[test]
    fn test_find_overlaps() {
        assert_eq!(
            0,
            find_overlaps(
                [[0, 1416], [0, 4001], [0, 2006], [0, 1351]],
                [[2662, 4001], [0, 4001], [0, 2006], [0, 1351]]
            )
        );
        // 1338 * 1910 * 2005 * 1350
        assert_eq!(
            6917316165000,
            find_overlaps(
                [[2662, 4001], [0, 4001], [0, 2006], [0, 1351]],
                [[0, 4001], [2090, 4001], [0, 4001], [0, 1351]]
            )
        );
    }

    #[test]
    fn test_day_19_number_of_combinations_of_accepted_ratings() {
        assert_eq!(
            167409079868000,
            day_19_number_of_combinations_of_accepted_ratings(EXAMPLE)
        );
    }
}
